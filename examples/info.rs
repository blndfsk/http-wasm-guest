//! Example plugin that logs HTTP request information.
//!
//! This plugin demonstrates how to use the http-wasm-guest API to
//! log request metadata, headers
//!
//! # Features
//! - Logs HTTP version, method, and URI
//! - Logs all request headers
//!
//! # Usage
//! Register the plugin and initialize logging in `main`.
use http_wasm_guest::{
    Guest,
    host::{Request, Response, admin, feature},
    register,
};

/// Plugin that logs request information.
struct Plugin;

impl Guest for Plugin {
    /// Handles incoming requests by logging metadata and headers.
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        log::info!("Request: {} {} {}", request.method(), request.version(), request.uri());
        for (key, value) in request.header().values() {
            log::info!("Header: {} [ {}]", key, value.iter().fold(String::new(), |acc, b| acc + &b.to_string() + " "));
        }
        log::info!("Body: {}", request.body().read());
        (true, 0)
    }
    fn handle_response(&self, _req_ctx: i32, _request: &Request, response: &Response, _is_error: bool) {
        log::info!("Status: {}", response.status());
        log::info!("Body: {}", response.body().read());
    }
}

fn main() {
    admin::init_log().expect("error initializing logger");
    admin::enable(feature::BufferRequest | feature::BufferResponse);
    let plugin = Plugin;
    register(plugin);
}
