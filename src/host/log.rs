//! Host-backed logging utilities for http-wasm guest plugins.
//!
//! This module provides functions for forwarding log messages to the host runtime.
//! By default, the `log` feature is enabled, which integrates the standard Rust `log` crate
//! and provides the [`HostLogger`](crate::HostLogger) implementation for ergonomic logging via macros like
//! `log::info!`, `log::warn!`, etc.
//!
//! ## Recommended Usage
//!
//! It is recommended to use the default `log` feature and the provided
//! [`HostLogger`](crate::HostLogger).
//! This allows you to leverage the Rust logging ecosystem and have messages automatically
//! forwarded to the host with proper filtering and formatting.
//!
//! Use [`HostLogger::init()`](crate::HostLogger::init), [`HostLogger::init_with_level()`](crate::HostLogger::init_with_level),
//! or [`HostLogger::init_with_config()`](crate::HostLogger::init_with_config)
//! to install the logger and configure the maximum log level / message length.
//! After initialization, all log records are filtered and sent to the host according to the configured level.
//! Log messages are formatted into a fixed-size buffer (2048 bytes) and are also capped by
//! `HostLoggerConfig::max_message_len` (default 2048).
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
//! You can then use the low-level functions [`write()`] and [`enabled()`] in this module for direct logging.
//!
//! ## Example (with feature = "log")
//!
//! ```no_run
//! use http_wasm_guest::HostLogger;
//! use log;
//!
//! let _ = HostLogger::init();
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
/// * `level` - The severity code to use for the log message, passed to the host
///   as-is. The host maps debug=-1, info=0, warn=1, error=2.
/// * `message` - The log message as a byte slice. It is passed to the host directly
///   from guest memory with no guest-side copy or allocation. When routed through
///   [`HostLogger`](crate::HostLogger), formatted messages are capped at 2048 bytes
///   (or `HostLoggerConfig::max_message_len`) and truncated with a marker.
///
/// This function is typically called internally by the logger implementation, but can be used directly to send custom log messages to the host.
///
/// # Example
///
/// ```no_run
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
/// Hosts may cache this value at request granularity, so calling it per
/// message is inexpensive but not guaranteed to observe mid-request changes.
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
