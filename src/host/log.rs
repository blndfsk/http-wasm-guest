//! Host-backed logging utilities for http-wasm guest plugins.
//!
//! This module provides functions for forwarding log messages to the host runtime.
//! By default, the `log` feature is enabled, which integrates the standard Rust `log` crate
//! and provides the [`HostLogger`] implementation for ergonomic logging via macros like
//! `log::info!`, `log::warn!`, etc.
//!
//! ## Recommended Usage
//!
//! It is recommended to use the default `log` feature and the provided [`HostLogger`].
//! This allows you to leverage the Rust logging ecosystem and have messages automatically
//! forwarded to the host with proper filtering and formatting.
//!
//! Use [`admin::init_log`] or [`admin::init_log_with_level`] to install the logger and configure the maximum log level.
//! After initialization, all log records are filtered and sent to the host according to the configured level.
//! Log messages are formatted into a fixed-size buffer and truncated if longer than 4096 bytes.
//!
//! ## Disabling the `log` Feature
//!
//! If you wish to disable the `log` integration (for a smaller binary or custom logging),
//! you can do so by specifying `default-features = false` in your dependency declaration:
//!
//! ```toml
//! http-wasm-guest = { version = "...", default-features = false }
//! ```
//!
//! You can then use the low-level functions [`write`] and [`enabled`] in this module for direct logging.
//!
//! ## Example (with feature = "log")
//!
//! ```no_run
//! use http_wasm_guest::host::admin;
//! use log;
//!
//! let _ = admin::init_log();
//! log::info!("Hello from plugin!");
//! log::warn!("Something might be wrong!");
//! ```
//!
//! ## Example (manual usage)
//!
//! ```no_run
//! use http_wasm_guest::host::log;
//!
//! if log::enabled(0) {
//!     log::write(0, b"Hello from plugin!");
//! }
//! ```
use crate::host::handler;

/// Forwards a log message to the host logger with the specified severity level.
///
/// # Arguments
///
/// * `level` - The severity code to use for the log message. This should match the host's expected log level mapping.
/// * `message` - The log message as a byte slice. Messages exceeding the host's buffer limit may be truncated.
///
/// This function is typically called internally by the logger implementation, but can be used directly to send custom log messages to the host.
///
/// # Example
///
/// ```rust
/// use http_wasm_guest::host::log;
/// log::write(0, b"Hello from plugin!");
/// ```
pub fn write(level: i32, message: &[u8]) {
    handler::log(level, message);
}

/// Checks if logging is enabled for the specified severity level.
///
/// # Arguments
///
/// * `level` - The severity code to check. This should match the host's expected log level mapping.
///
/// # Returns
///
/// `true` if logging is enabled for the given level; otherwise, `false`.
///
/// # Example
///
/// ```no_run
/// use http_wasm_guest::host::log;
/// if log::enabled(0) {
///     log::write(0, b"Info-level log message");
/// }
/// ```
pub fn enabled(level: i32) -> bool {
    handler::log_enabled(level)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handler_log_call() {
        // Should not panic - mock accepts any log call
        write(2, b"test log message");
    }
    #[test]
    fn log_enabled_check() {
        // The mock enables levels 0-3 (Error, Warn, Info, Debug)
        assert!(!enabled(-2)); // Trace (disabled)
        assert!(!enabled(-1)); // Debug (disabled)
        assert!(enabled(0)); // Info
        assert!(enabled(1)); // Warn
        assert!(enabled(2)); // Error
        assert!(enabled(3)); // Fatal
        assert!(enabled(4)); // Panic
    }
}
