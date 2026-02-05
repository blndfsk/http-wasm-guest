#![allow(clippy::needless_doctest_main)]
#![warn(missing_docs)]

//! HTTP WebAssembly guest library for building http-wasm plugins.

use std::sync::OnceLock;

/// Core HTTP types and traits used by guest plugins.
pub mod api;
/// Host interface for requests, responses, logging, and feature management.
pub mod host;

struct Handler {
    guest: Box<dyn Guest>,
}
unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

static GUEST: OnceLock<Handler> = OnceLock::new();

/// Boxed HTTP request trait object used by guest handlers.
pub type Request = Box<dyn api::Request + 'static>;
/// Boxed HTTP response trait object used by guest handlers.
pub type Response = Box<dyn api::Response + 'static>;


/// Trait implemented by guest plugins to handle HTTP requests and responses.
pub trait Guest {
    /// Handle an incoming request. Return `(true, ctx)` to continue processing.
    fn handle_request(&self, _request: Request, _response: Response) -> (bool, i32) {
        (true, 0)
    }

    /// Handle an outgoing response after upstream processing completes.
    fn handle_response(&self, _request: Request, _response: Response) {}
}

/// Register a guest plugin implementation with the runtime.
pub fn register<T: Guest + 'static>(guest: T) {
    GUEST.get_or_init(|| Handler {
        guest: Box::new(guest),
    });
}

#[unsafe(export_name = "handle_request")]
fn http_request() -> i64 {
    let (next, ctx_next) = match GUEST.get() {
        Some(handler) => handler.guest.handle_request(Request::default(),Response::default()),
        None => (true, 0),
    };

    if next { (ctx_next as i64) << 32 | 1 } else { 0 }
}

#[unsafe(export_name = "handle_response")]
fn http_response(_req_ctx: i32, _is_error: i32) {
    if let Some(handler) = GUEST.get() {
        handler.guest.handle_response(Request::default(),Response::default())
    };
}
