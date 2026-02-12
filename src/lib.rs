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
    fn handle_response(&self, _request: &Request, _response: &Response) {}
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
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler.guest.handle_response(&REQ, &RES)
    };
}
