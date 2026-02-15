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
    host::{self, Request, Response},
    register,
};
use log::info;

/// Plugin that logs request information.
struct Plugin;

impl Guest for Plugin {
    /// Handles incoming requests by logging metadata and headers.
    fn handle_request(&self, request: &Request, _response: &Response) -> (bool, i32) {
        info!("Request: {} {} {}", request.method(), request.version(), request.uri());
        for (key, value) in request.header().values() {
            info!(
                "Header: {} [ {}]",
                key,
                value.iter().fold(String::new(), |acc, b| acc + &b.to_string() + " ")
            );
        }
        (true, 0)
    }
}

fn main() {
    host::log::init().expect("error initializing logger");
    let plugin = Plugin;
    register(plugin);
}
