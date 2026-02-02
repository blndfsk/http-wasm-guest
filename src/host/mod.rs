//! Host interface module for http-wasm guest plugins.
//!
//! This module provides the main interface between WebAssembly guest modules and
//! the http-wasm host environment. It contains types and functions for handling
//! HTTP requests and responses, managing features, and logging.
//!
//! # Submodules
//!
//! - [`feature`] - Enable optional host features like body buffering
//! - [`log`] - Logging functionality that routes through the host

pub mod feature;
pub mod log;

pub(crate) mod request;
pub(crate) mod response;

mod body;
mod handler;
mod header;

use crate::api::{Body, Bytes, Header};

pub(crate) struct Message {
    header: Box<dyn Header>,
    body: Box<dyn Body>,
}
impl Message {
    pub fn new(kind: i32) -> Self {
        Self {
            header: Box::new(header::Header(kind)),
            body: Box::new(body::Body(kind)),
        }
    }
}

/// Retrieves the configuration data provided by the host.
///
/// This function returns the configuration that was passed to the WebAssembly module
/// by the host environment. The configuration is typically provided as binary data
/// that can contain JSON, YAML, TOML, or other configuration formats.
///
/// # Returns
///
/// Returns a [`Bytes`] object containing the raw configuration data. Use the
/// [`Bytes::to_str()`] method to convert to a UTF-8 string if needed.
///
/// # Examples
///
/// ```no_run
/// use http_wasm_guest::host::config;
///
/// // Get configuration as bytes
/// let config_bytes = config();
///
/// // Convert to string for text-based config formats
/// match config_bytes.to_str() {
///     Ok(config_str) => {
///         // Parse configuration (e.g., JSON)
///         println!("Plugin config: {}", config_str);
///
///         // Example with JSON parsing
///         // let config: serde_json::Value = serde_json::from_str(config_str)?;
///     }
///     Err(e) => {
///         // Handle invalid UTF-8 configuration
///         eprintln!("Config is not valid UTF-8: {}", e);
///
///         // Still possible to work with raw bytes
///         println!("Config size: {} bytes", config_bytes.len());
///     }
/// }
///
/// // Direct access to raw bytes
/// let raw_data: &[u8] = &config_bytes;
/// ```
///
/// # Notes
///
/// - The configuration is set by the host environment when the WebAssembly module is loaded
/// - If no configuration was provided, this function returns empty [`Bytes`]
/// - The configuration format depends on the host implementation and use case
/// - Common formats include JSON for structured data, but any binary format is supported
pub fn config() -> Bytes {
    Bytes::from(handler::get_config())
}

#[cfg(test)]
mod tests {}
