#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]

//! A Rust library for implementing HTTP WebAssembly guest plugins.
//!
//! This library provides a complete implementation of the [http-wasm Guest ABI](https://http-wasm.io/http-handler-abi/)
//! for building WebAssembly modules that can process HTTP requests and responses in compatible host environments
//! such as [Traefik](https://traefik.io/) and other http-wasm enabled proxies.
//!
//! # Quick Start
//!
//! Create a plugin by implementing the [`Guest`] trait and registering it:
//!
//! ```no_run
//! use http_wasm_guest::{Guest, host::{Request, Response}, register};
//!
//! struct MyPlugin;
//!
//! impl Guest for MyPlugin {
//!     fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
//!         // Add a custom header to all requests
//!         request.header().add(b"X-Plugin", b"MyPlugin-v1.0");
//!         (true, 0) // Continue to next handler
//!     }
//!
//!     fn handle_response(&self, _request: Request, response: Response) {
//!         // Add security headers to all responses
//!         response.header().add(b"X-Content-Type-Options", b"nosniff");
//!     }
//! }
//!
//! fn main() {
//!     register(MyPlugin);
//! }
//! ```
//!
//! # Building
//!
//! Compile your plugin to WebAssembly using:
//!
//! ```bash
//! cargo build --target wasm32-wasip1 --release
//! ```
//!
//! # Core Concepts
//!
//! ## Guest Trait
//!
//! The [`Guest`] trait is the main interface for your plugin. It provides two methods:
//! - [`Guest::handle_request`] - Called for each incoming request
//! - [`Guest::handle_response`] - Called for each outgoing response
//!
//! ## Request/Response Processing
//!
//! Your plugin can:
//! - Inspect and modify HTTP headers
//! - Read and write request/response bodies
//! - Change request methods and URIs
//! - Set response status codes
//! - Access client information (IP address, etc.)
//!
//! ## Features
//!
//! Enable optional host features for advanced functionality:
//!
//! ```no_run
//! use http_wasm_guest::host::feature::{enable, BufferRequest, BufferResponse};
//! use http_wasm_guest::{Guest, register};
//!
//! struct MyPlugin;
//! impl Guest for MyPlugin {}
//!
//! fn main() {
//!     // Enable body buffering for modification
//!     enable(BufferRequest | BufferResponse);
//!
//!     register(MyPlugin);
//! }
//! ```
//!
//! ## Logging
//!
//! Set up logging to debug your plugin:
//!
//! ```no_run
//! use http_wasm_guest::{Guest, host, register};
//! use log::info;
//!
//! struct MyPlugin;
//! impl Guest for MyPlugin {}
//!
//! fn main() {
//!     host::log::init().expect("Failed to initialize logger");
//!     info!("Plugin starting up");
//!
//!     register(MyPlugin);
//! }
//! ```
//!
//! # Modules
//!
//! - [`host`] - Interface to the host environment (requests, responses, logging, etc.)
//!
//! # Examples
//!
//! See the `examples/` directory for complete plugin implementations.

use std::sync::OnceLock;

use host::{Request, Response};

pub mod host;

struct Handler {
    guest: Box<dyn Guest>,
}
unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

static GUEST: OnceLock<Handler> = OnceLock::new();

/// The main trait for implementing HTTP WebAssembly guest plugins.
///
/// This trait defines the interface between the WebAssembly guest module and the host
/// environment (such as Traefik). Implementations of this trait can intercept and modify
/// HTTP requests and responses as they flow through the host's request processing pipeline.
///
/// The trait follows the [http-wasm Guest ABI](https://http-wasm.io/http-handler-abi/) specification.
///
/// # Lifecycle
///
/// 1. **Request Phase**: The host calls [`handle_request`] when an HTTP request arrives
/// 2. **Response Phase**: The host calls [`handle_response`] when an HTTP response is ready
///
/// # Example
///
/// ```no_run
/// use http_wasm_guest::{Guest, host::{Request, Response}, register};
///
/// struct MyPlugin;
///
/// impl Guest for MyPlugin {
///     fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
///         // Add a custom header to the request
///         request.header().add(b"X-Custom-Header", b"MyValue");
///
///         // Continue to next handler with context 0
///         (true, 0)
///     }
///
///     fn handle_response(&self, _request: Request, response: Response) {
///         // Modify the response status
///         response.set_status(200);
///     }
/// }
///
/// fn main() {
///     register(MyPlugin);
/// }
/// ```
///
/// [`handle_request`]: Guest::handle_request
/// [`handle_response`]: Guest::handle_response
pub trait Guest {
    /// Called by the host when processing an incoming HTTP request.
    ///
    /// This method is invoked early in the request processing pipeline, allowing the plugin
    /// to inspect and modify both the request and response objects before they are passed
    /// to subsequent handlers.
    ///
    /// # Parameters
    ///
    /// - `request`: The incoming HTTP request that can be inspected and modified
    /// - `response`: The response object that can be pre-populated or modified
    ///
    /// # Returns
    ///
    /// Returns a tuple `(bool, i32)` where:
    /// - The `bool` indicates whether processing should continue:
    ///   - `true`: Continue to the next handler in the chain
    ///   - `false`: Skip remaining handlers and use the current response
    /// - The `i32` is a context value passed to [`handle_response`] for correlation
    ///
    /// # Default Implementation
    ///
    /// The default implementation does nothing and returns `(true, 0)`, allowing
    /// request processing to continue normally.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use http_wasm_guest::{Guest, host::{Request, Response}};
    /// # struct MyPlugin;
    /// impl Guest for MyPlugin {
    ///     fn handle_request(&self, request: Request, response: Response) -> (bool, i32) {
    ///         // Log request method and URI
    ///         let method = request.method();
    ///         let uri = request.uri();
    ///
    ///         // Block requests to /admin path
    ///         if uri.to_str().unwrap_or("").starts_with("/admin") {
    ///             response.set_status(403);
    ///             response.body().write(b"Forbidden");
    ///             return (false, 1); // Stop processing, return 403
    ///         }
    ///
    ///         // Add request ID for tracking
    ///         request.header().add(b"X-Request-ID", b"12345");
    ///
    ///         (true, 0) // Continue processing
    ///     }
    /// }
    /// ```
    ///
    /// [`handle_response`]: Guest::handle_response
    fn handle_request(&self, _request: Request, _response: Response) -> (bool, i32) {
        (true, 0)
    }

    /// Called by the host when processing the HTTP response.
    ///
    /// This method is invoked after the request has been processed by the upstream
    /// handlers, allowing the plugin to inspect and modify the final response before
    /// it is sent back to the client.
    ///
    /// # Parameters
    ///
    /// - `request`: The original HTTP request (read-only access for context)
    /// - `response`: The HTTP response that can be inspected and modified
    ///
    /// # Default Implementation
    ///
    /// The default implementation does nothing, leaving the response unchanged.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use http_wasm_guest::{Guest, host::{Request, Response}};
    /// # struct MyPlugin;
    /// impl Guest for MyPlugin {
    ///     fn handle_response(&self, request: Request, response: Response) {
    ///         // Add security headers to all responses
    ///         response.header().add(b"X-Content-Type-Options", b"nosniff");
    ///         response.header().add(b"X-Frame-Options", b"DENY");
    ///
    ///         // Log response status
    ///         let status = response.status();
    ///         if status >= 400 {
    ///             // Customize error responses
    ///             response.body().write(b"Custom error page");
    ///         }
    ///
    ///         // Add correlation header using request info
    ///         let method = request.method();
    ///         response.header().add(b"X-Request-Method", &method);
    ///     }
    /// }
    /// ```
    fn handle_response(&self, _request: Request, _response: Response) {}
}

/// Registers a guest plugin implementation with the http-wasm runtime.
///
/// This function must be called exactly once in the `main` function of your WebAssembly
/// module to register your [`Guest`] implementation with the host environment.
///
/// # Parameters
///
/// - `guest`: An instance of your plugin that implements the [`Guest`] trait
///
/// # Panics
///
/// This function uses [`OnceLock::get_or_init`] internally, so calling it multiple times
/// will not panic, but only the first registration will take effect.
///
/// # Example
///
/// ```rust
/// use http_wasm_guest::{Guest, host::{Request, Response}, register};
///
/// struct MyPlugin {
///     config: String,
/// }
///
/// impl Guest for MyPlugin {
///     fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
///         // Plugin logic here
///         (true, 0)
///     }
/// }
///
/// fn main() {
///     let plugin = MyPlugin {
///         config: "example".to_string(),
///     };
///     register(plugin);
/// }
/// ```
///
/// # Notes
///
/// - The registered plugin will be called by the host for each HTTP request/response
/// - The plugin instance must be `'static` and will live for the entire duration of the module
/// - This function should be called in `main()` before the module completes initialization
///
/// [`Guest`]: Guest
/// [`OnceLock::get_or_init`]: std::sync::OnceLock::get_or_init
pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler {
        guest: Box::new(guest),
    });
}

#[unsafe(export_name = "handle_request")]
fn http_request() -> i64 {
    let (next, ctx_next) = match GUEST.get() {
        Some(handler) => handler
            .guest
            .handle_request(Request::default(), Response::default()),
        None => (true, 0),
    };

    if next { (ctx_next as i64) << 32 | 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler
            .guest
            .handle_response(Request::default(), Response::default())
    };
}
