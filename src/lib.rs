#![warn(missing_docs)]

//! HTTP WebAssembly guest library for building `http-wasm` plugins.
//!
//! This crate exposes a guest-facing API for inspecting and mutating HTTP
//! requests and responses within a host runtime. Implement [`Guest`] and
//! call [`register`] to wire up your plugin entry points.

use std::sync::{LazyLock, OnceLock};

use crate::host::{Request, Response};

/// Host interface for requests, responses, logging, and feature management.
pub mod host;

struct Handler {
    guest: Box<dyn Guest>,
}
unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

/// Trait implemented by guest plugins to handle HTTP requests and responses.
///
/// Implement this trait to observe and modify inbound requests and outbound
/// responses. The runtime constructs [`Request`] and [`Response`] handles
/// for each request cycle and forwards them to your implementation.
pub trait Guest {
    /// Handle an incoming request before upstream processing.
    ///
    /// Return `(true, ctx)` to continue the request to the upstream, passing
    /// an optional `ctx` value that will be provided to response handling.
    /// Returning `(false, _)` stops processing and short-circuits the request.
    fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
        (true, 0)
    }

    /// Handle an outgoing response after upstream processing completes.
    ///
    /// Use this hook to inspect or mutate headers and body before the response
    /// is sent back to the client.
    fn handle_response(&self, _req_ctx: i32, _request: &Request, _response: &Response, _is_error: bool) {}
}

static GUEST: OnceLock<Handler> = OnceLock::new();
static REQ: LazyLock<Request> = LazyLock::new(Request::default);
static RES: LazyLock<Response> = LazyLock::new(Response::default);

/// Register a guest plugin implementation with the runtime.
///
/// Call this once from your guest module initialization to install your
/// [`Guest`] implementation. Subsequent calls are ignored.
pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler { guest: Box::new(guest) });
}

#[unsafe(export_name = "handle_request")]
fn http_request() -> i64 {
    let (next, ctx_next) = match GUEST.get() {
        Some(handler) => handler.guest.handle_request(&REQ, &RES),
        None => (true, 0),
    };

    if next { (ctx_next as i64) << 32 | 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(req_ctx: i32, is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler.guest.handle_response(req_ctx, &REQ, &RES, is_error == 1)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::{admin, feature};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};

    // =========================================================================
    // Guest Trait Tests
    // =========================================================================

    struct TestPlugin {
        request_handled: Arc<AtomicBool>,
        response_handled: Arc<AtomicBool>,
        continue_request: bool,
        ctx_value: i32,
    }

    impl Guest for TestPlugin {
        fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
            self.request_handled.store(true, Ordering::SeqCst);
            (self.continue_request, self.ctx_value)
        }

        fn handle_response(&self, _req_ctx: i32, _request: &Request, _response: &Response, _is_error: bool) {
            self.response_handled.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn guest_default_implementation() {
        struct DefaultGuest;
        impl Guest for DefaultGuest {}

        let guest = DefaultGuest;
        let request = Request::default();
        let response = Response::default();

        let (cont, ctx) = guest.handle_request(&request, &response);
        assert!(cont);
        assert_eq!(ctx, 0);
    }

    #[test]
    fn guest_custom_implementation() {
        let request_handled = Arc::new(AtomicBool::new(false));
        let response_handled = Arc::new(AtomicBool::new(false));

        let plugin = TestPlugin {
            request_handled: request_handled.clone(),
            response_handled: response_handled.clone(),
            continue_request: true,
            ctx_value: 42,
        };

        let request = Request::default();
        let response = Response::default();

        let (cont, ctx) = plugin.handle_request(&request, &response);
        assert!(cont);
        assert_eq!(ctx, 42);
        assert!(request_handled.load(Ordering::SeqCst));

        plugin.handle_response(ctx, &request, &response, false);
        assert!(response_handled.load(Ordering::SeqCst));
    }

    #[test]
    fn guest_stop_request() {
        struct StopPlugin;
        impl Guest for StopPlugin {
            fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
                (false, 0)
            }
        }

        let plugin = StopPlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(!cont);
    }

    #[test]
    fn guest_context_passing() {
        let ctx_received = Arc::new(AtomicI32::new(0));
        let ctx_clone = ctx_received.clone();

        struct ContextPlugin {
            ctx_received: Arc<AtomicI32>,
        }

        impl Guest for ContextPlugin {
            fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
                (true, 12345)
            }

            fn handle_response(&self, req_ctx: i32, _request: &Request, _response: &Response, _is_error: bool) {
                self.ctx_received.store(req_ctx, Ordering::SeqCst);
            }
        }

        let plugin = ContextPlugin { ctx_received: ctx_clone };
        let request = Request::default();
        let response = Response::default();

        let (_, ctx) = plugin.handle_request(&request, &response);
        plugin.handle_response(ctx, &request, &response, false);

        assert_eq!(ctx_received.load(Ordering::SeqCst), 12345);
    }

    // =========================================================================
    // End-to-End Plugin Scenario Tests
    // =========================================================================

    /// Simulates a plugin that adds a custom header to requests
    struct HeaderAddPlugin;

    impl Guest for HeaderAddPlugin {
        fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
            request.header().add(b"X-Custom-Plugin", b"http-wasm-guest");
            (true, 0)
        }
    }

    #[test]
    fn e2e_header_plugin() {
        let plugin = HeaderAddPlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(cont);
    }

    /// Simulates a plugin that logs request info and continues
    struct LoggingPlugin;

    impl Guest for LoggingPlugin {
        fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
            let _method = request.method();
            let _uri = request.uri();
            let _version = request.version();
            let _headers = request.header().values();
            (true, 0)
        }
    }

    #[test]
    fn e2e_logging_plugin() {
        let plugin = LoggingPlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(cont);
    }

    /// Simulates a plugin that modifies the response
    struct ResponseModifierPlugin;

    impl Guest for ResponseModifierPlugin {
        fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
            (true, 1)
        }

        fn handle_response(&self, _req_ctx: i32, _request: &Request, response: &Response, _is_error: bool) {
            response.set_status(201);
            response.header().set(b"X-Modified", b"true");
            response.body().write(b"Modified response");
        }
    }

    #[test]
    fn e2e_response_modifier_plugin() {
        let plugin = ResponseModifierPlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, ctx) = plugin.handle_request(&request, &response);
        assert!(cont);
        assert_eq!(ctx, 1);

        plugin.handle_response(ctx, &request, &response, false);
    }

    /// Simulates a plugin that blocks certain requests
    struct BlockingPlugin {
        blocked_paths: Vec<&'static str>,
    }

    impl Guest for BlockingPlugin {
        fn handle_request(&self, request: &Request, response: &Response) -> (bool, i32) {
            let uri = request.uri();
            let uri_str = uri.to_str().unwrap_or("");

            for blocked in &self.blocked_paths {
                if uri_str.contains(blocked) {
                    response.set_status(403);
                    response.body().write(b"Forbidden");
                    return (false, 0);
                }
            }
            (true, 0)
        }
    }

    #[test]
    fn e2e_blocking_plugin_allows() {
        let plugin = BlockingPlugin { blocked_paths: vec!["/admin", "/secret"] };
        let request = Request::default();
        let response = Response::default();

        // Mock returns "https://test" which doesn't contain blocked paths
        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(cont);
    }

    /// Simulates a plugin that uses configuration
    struct ConfigurablePlugin;

    impl Guest for ConfigurablePlugin {
        fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
            let config = admin::config();
            if config.to_str().unwrap_or("").contains("config") {
                request.header().add(b"X-Config-Loaded", b"true");
            }
            (true, 0)
        }
    }

    #[test]
    fn e2e_configurable_plugin() {
        let plugin = ConfigurablePlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(cont);
    }

    /// Simulates a plugin that enables features
    struct FeatureEnablingPlugin;

    impl Guest for FeatureEnablingPlugin {
        fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
            admin::enable(feature::BufferRequest | feature::BufferResponse);
            (true, 0)
        }
    }

    #[test]
    fn e2e_feature_enabling_plugin() {
        let plugin = FeatureEnablingPlugin;
        let request = Request::default();
        let response = Response::default();

        let (cont, _) = plugin.handle_request(&request, &response);
        assert!(cont);
    }

    /// Simulates a complete request/response cycle
    struct FullCyclePlugin {
        pub request_count: AtomicU32,
    }

    impl Guest for FullCyclePlugin {
        fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
            let count = self.request_count.fetch_add(1, Ordering::SeqCst);

            // Log request details
            let _method = request.method();
            let _uri = request.uri();
            let _source = request.source_addr();

            // Add tracking header
            request.header().add(b"X-Request-Id", format!("{}", count).as_bytes());

            (true, count as i32)
        }

        fn handle_response(&self, req_ctx: i32, _request: &Request, response: &Response, is_error: bool) {
            if !is_error {
                response.header().set(b"X-Processed-By", b"FullCyclePlugin");
                response.header().add(b"X-Request-Context", format!("{}", req_ctx).as_bytes());
            }
        }
    }

    #[test]
    fn e2e_full_cycle_plugin() {
        let plugin = FullCyclePlugin { request_count: AtomicU32::new(0) };
        let request = Request::default();
        let response = Response::default();

        // First request
        let (cont1, ctx1) = plugin.handle_request(&request, &response);
        assert!(cont1);
        assert_eq!(ctx1, 0);
        plugin.handle_response(ctx1, &request, &response, false);

        // Second request
        let (cont2, ctx2) = plugin.handle_request(&request, &response);
        assert!(cont2);
        assert_eq!(ctx2, 1);
        plugin.handle_response(ctx2, &request, &response, false);
    }

    // =========================================================================
    // Registration Tests
    // =========================================================================

    struct SimplePlugin;

    impl Guest for SimplePlugin {
        fn handle_request(&self, _request: &Request, _response: &Response) -> (bool, i32) {
            (true, 0)
        }
    }

    #[test]
    fn register_plugin() {
        // Note: register uses OnceLock, so this will only work once per test run
        // Additional calls are ignored
        let plugin = SimplePlugin;
        register(plugin);
    }

    // =========================================================================
    // Entry Point Tests (http_request and http_response)
    // =========================================================================

    #[test]
    fn http_request_returns_continue_with_context() {
        // When a guest is registered and returns (true, ctx), the result should be
        // (ctx << 32) | 1
        let result = http_request();
        // The lower bit should be 1 (continue)
        assert_eq!(result & 1, 1);
    }

    #[test]
    fn http_response_does_not_panic() {
        // Should not panic when called with various parameters
        http_response(0, 0);
        http_response(42, 0);
        http_response(0, 1);
        http_response(123, 1);
    }

    #[test]
    fn http_request_without_guest_returns_default() {
        // When no guest is registered (or default behavior), should return continue
        // This tests the None branch - though in practice GUEST is set by other tests
        let result = http_request();
        // Should still have the continue bit set
        assert_eq!(result & 1, 1);
    }
}
