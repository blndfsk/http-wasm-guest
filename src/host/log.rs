//! Logging functionality for http-wasm guest plugins.
//!
//! This module provides logging capabilities that integrate with the http-wasm host
//! environment. Log messages are forwarded to the host, which handles the actual
//! output according to its configuration.
//!
//! # Setup
//!
//! Initialize logging early in your plugin's `main()` function:
//!
//! ```no_run
//! use http_wasm_guest::host;
//! use log::info;
//!
//! fn main() {
//!     // Initialize with default Info level
//!     host::log::init().expect("Failed to initialize logger");
//!
//!     // Or initialize with a specific level
//!     host::log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
//!
//!     // Now you can use standard Rust logging macros
//!     log::info!("Plugin initialized");
//!     log::warn!("This is a warning");
//!     log::error!("This is an error");
//! }
//! ```
//!
//! # Log Levels
//!
//! The following log levels are supported (from most to least verbose):
//! - `Error` - Error messages
//! - `Warn` - Warning messages
//! - `Info` - Informational messages (default)
//! - `Debug` - Debug messages (not supported by all hosts)
//! - `Trace` - Trace messages (not supported by all hosts)
//!
//! # Host Integration
//!
//! - Log messages are sent to the http-wasm host for processing
//! - The host determines the final output format and destination
//! - Some hosts may not support all log levels (Debug/Trace are often disabled)
//! - Log output depends on the host's logging configuration

use log::{Level, Log, Metadata, Record, SetLoggerError};

use super::handler;

static LOGGER: HostLogger = HostLogger {};
static LVL: [i32; 6] = [3, 2, 1, 0, -1, -1];

fn map(level: Level) -> i32 {
    LVL[level as usize]
}
struct HostLogger {}

impl Log for HostLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level() && handler::log_enabled(map(metadata.level()))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        handler::log(
            map(record.metadata().level()),
            format!("{}", record.args()).as_bytes(),
        );
    }

    fn flush(&self) {}
}

/// Initializes the http-wasm logger with a specific log level.
///
/// This function sets up logging to route log messages through the http-wasm host
/// environment. The host will handle the actual logging output according to its
/// own configuration.
///
/// # Parameters
///
/// - `level`: The maximum log level to enable (e.g., `Level::Info`, `Level::Debug`)
///
/// # Returns
///
/// Returns `Ok(())` if the logger was successfully initialized, or a `SetLoggerError`
/// if a logger was already set.
///
/// # Example
///
/// ```no_run
/// use http_wasm_guest::host::log::init_with_level;
/// use log::Level;
///
/// fn main() {
///     init_with_level(Level::Debug).expect("Failed to initialize logger");
///     log::info!("Logger initialized with debug level");
/// }
/// ```
///
/// # Notes
///
/// - This should be called early in your plugin's `main()` function
/// - Only call this once per module - subsequent calls will return an error
/// - The actual log output format and destination depends on the host implementation
#[inline]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes the http-wasm logger with the default Info log level.
///
/// This is a convenience function that calls [`init_with_level`] with `Level::Info`.
/// Use this when you want standard logging without debug messages.
///
/// # Returns
///
/// Returns `Ok(())` if the logger was successfully initialized, or a `SetLoggerError`
/// if a logger was already set.
///
/// # Example
///
/// ```no_run
/// use http_wasm_guest::host::log::init;
///
/// fn main() {
///     init().expect("Failed to initialize logger");
///     log::info!("Logger initialized");
///     log::warn!("This will be logged");
///     log::debug!("This will NOT be logged (below Info level)");
/// }
/// ```
///
/// # Notes
///
/// - Equivalent to calling `init_with_level(Level::Info)`
/// - This should be called early in your plugin's `main()` function
/// - Only call this once per module - subsequent calls will return an error
///
/// [`init_with_level`]: init_with_level
#[inline]
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(-1, map(Level::Trace));
        assert_eq!(-1, map(Level::Debug));
        assert_eq!(0, map(Level::Info));
        assert_eq!(1, map(Level::Warn));
        assert_eq!(2, map(Level::Error));
    }
}
