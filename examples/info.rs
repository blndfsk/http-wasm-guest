//! Example plugin that logs HTTP request information.
//!
//! This plugin demonstrates how to use the http-wasm-guest API to
//! log request metadata, headers and body
use http_wasm_guest::{
    Guest, HostLogger,
    host::{Request, Response, admin, feature},
    register,
};
use log::info;

/// Plugin that logs request information.
struct Plugin {}

impl Guest for Plugin {
    /// Handles incoming requests by logging metadata and headers.
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        info!("Request: {} {} {} {}", request.method(), request.version(), request.uri(), request.source_addr());
        for (name, values) in request.header.entries_iter() {
            let values = values.iter().map(|v| format!("{v}")).collect::<Vec<_>>().join(", ");
            info!("Header: {} [{}]", name, values);
        }
        info!("Body: {}", request.body.read());
        (true, 0)
    }
    /// Handles outgoing responses by logging status and body.
    fn handle_response(&self, _req_ctx: i32, _request: &Request, response: &Response, _is_error: bool) {
        info!("Status: {}", response.status());
    }
}

fn main() {
    let _ = HostLogger::init();
    admin::enable(feature::BufferRequest | feature::BufferResponse);
    let plugin = Plugin {};
    register(plugin);
}
