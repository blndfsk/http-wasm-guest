//! This example shows how to add a new header to the request
//!
//! # Example
//! This plugin demonstrates how to use the http-wasm-guest API to
//! add a custom header (`X-Bar: bar`) to incoming HTTP requests.
//! The plugin implements the `Guest` trait and registers itself
//! in the `main` function.

use http_wasm_guest::{
    Guest,
    host::{Request, Response},
    register,
};

/// A simple plugin that adds a custom header to each request.
struct Plugin;

impl Guest for Plugin {
    /// Handles incoming requests by adding the `X-Bar: bar` header.
    ///
    /// # Arguments
    /// * `request` - The incoming HTTP request.
    /// * `_response` - The HTTP response (unused in this example).
    ///
    /// # Returns
    /// Returns a tuple `(true, 0)` to indicate the request should continue.
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        let header = request.header();
        header.add(b"X-Bar", b"bar");
        (true, 0)
    }
}

/// Registers the plugin with the http-wasm runtime.
fn main() {
    let plugin = Plugin;
    register(plugin);
}
