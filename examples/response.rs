//! Example plugin that modifies the HTTP response body.
//!
//! This plugin demonstrates how to use the http-wasm-guest API to
//! buffer and modify the response body in the response phase.
//!
//! # Features
//! - Buffers the response body using `BufferResponse`
//! - Overwrites the response body with the string "test"
//!
//! # Usage
//! Register the plugin and enable response buffering in `main`.

use http_wasm_guest::{
    Guest, Request, Response, host::feature::{self, BufferResponse}, register
};

/// Plugin that overwrites the response body.
struct Plugin;

impl Guest for Plugin {
    /// Handles the response by writing "test" to the body.
    fn handle_response(&self, _request: Request, response: Response) {
        response.body().write(b"test");
    }
}

fn main() {
    // Enable response buffering and register the plugin.
    let plugin = Plugin;
    feature::enable(BufferResponse);
    register(plugin);
}
