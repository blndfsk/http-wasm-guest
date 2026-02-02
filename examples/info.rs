//! Example plugin that logs HTTP request information.
//!
//! This plugin demonstrates how to use the http-wasm-guest API to
//! log request metadata, headers, and enable request body buffering.
//!
//! # Features
//! - Logs HTTP version, method, and URI
//! - Logs all request headers
//! - Enables buffering of the request body (via BufferRequest)
//!
//! # Usage
//! Register the plugin and initialize logging in `main`.

use http_wasm_guest::{
    Guest, Request, Response, host::{self, feature}, register
};
use log::info;

/// Plugin that logs request information.
struct Plugin;

impl Guest for Plugin {
    /// Handles incoming requests by logging metadata and headers.
    fn handle_request(&self, request: Request, _response: Response) -> (bool, i32) {
        info!(
            "{} {} {}",
            request.version(),
            request.method(),
            request.uri()
        );
        info!("{:?}", request.header().get());
        (true, 0)
    }
}

fn main() {
    // Initialize logger and enable request body buffering.
    host::log::init().expect("error initializing logger");
    feature::enable(feature::BufferRequest);
    let plugin = Plugin;
    register(plugin);
}
