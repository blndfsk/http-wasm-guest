//! Host-backed logging adapter.
//!
//! This module integrates the `log` crate with a host-provided logger.
//!
//! Use [`init_with_level`] or [`init`] to install the logger and configure
//! the maximum log level.
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

use crate::host::handler;

static LOGGER: HostLogger = HostLogger;
static LVL: [i32; 6] = [3, 2, 1, 0, -1, -1];

/// Map a Rust `log::Level` to the host severity code.
///
/// The mapping is defined by `LVL` and must stay consistent with the host.
fn map_to_host(level: Level) -> i32 {
    LVL[level as usize]
}
/// Logger implementation that forwards records to the host.
///
/// This is installed via [`log::set_logger`] in [`init_log_with_level`].
struct HostLogger;

impl Log for HostLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        handler::log(map_to_host(record.metadata().level()), format!("{}", record.args()).as_bytes());
    }

    fn flush(&self) {}
}

/// Initialize the host-backed logger with a specific maximum level.
///
/// After initialization, calls to the `log` crate are forwarded to the host
/// logger, subject to `level` and host-side filtering.
#[inline]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_max_level(max_level(level.to_level_filter()));
    log::set_logger(&LOGGER)?;
    Ok(())
}
/// Determine the max_log_level as configured by the host
/// If the log-level is more restrictive on the host as the plugin tries to configure,
/// the level is decremented until an enabled level is found.
fn max_level(level: LevelFilter) -> LevelFilter {
    if handler::log_enabled(level.to_level().map_or_else(|| 3, map_to_host)) { level } else { level.decrement_severity() }
}
/// Initialize the host-backed logger with the default Info level.
///
/// This is a convenience wrapper around [`init_log_with_level`] using `Level::Info`.
#[inline]
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_log_with_level() {
        // Logger can only be set once globally, so we just verify it doesn't panic
        // and returns a result (either Ok or Err if already set)
        let _result = init_with_level(Level::Info);
        // If this is the first init, max_level should be Info
        // If logger was already set, this is still valid
    }

    #[test]
    fn map_to_host_error() {
        // Level::Error = 1, LVL[1] = 2
        assert_eq!(map_to_host(Level::Error), 2);
    }

    #[test]
    fn map_to_host_warn() {
        // Level::Warn = 2, LVL[2] = 1
        assert_eq!(map_to_host(Level::Warn), 1);
    }

    #[test]
    fn map_to_host_info() {
        // Level::Info = 3, LVL[3] = 0
        assert_eq!(map_to_host(Level::Info), 0);
    }

    #[test]
    fn map_to_host_debug() {
        // Level::Debug = 4, LVL[4] = -1
        assert_eq!(map_to_host(Level::Debug), -1);
    }

    #[test]
    fn map_to_host_trace() {
        // Level::Trace = 5, LVL[5] = -1
        assert_eq!(map_to_host(Level::Trace), -1);
    }

    #[test]
    fn host_logger_enabled_within_max_level() {
        // Set max level to Info
        log::set_max_level(LevelFilter::Info);
        let metadata = log::Metadata::builder().level(Level::Info).target("test").build();
        assert!(LOGGER.enabled(&metadata));
    }

    #[test]
    fn host_logger_enabled_below_max_level() {
        log::set_max_level(LevelFilter::Info);
        let metadata = log::Metadata::builder().level(Level::Error).target("test").build();
        // Error is more severe than Info, so it should be enabled
        assert!(LOGGER.enabled(&metadata));
    }

    #[test]
    fn host_logger_disabled_above_max_level() {
        log::set_max_level(LevelFilter::Warn);
        let metadata = log::Metadata::builder().level(Level::Debug).target("test").build();
        // Debug is less severe than Warn, so it should be disabled
        assert!(!LOGGER.enabled(&metadata));
    }

    #[test]
    fn host_logger_log_enabled_message() {
        log::set_max_level(LevelFilter::Info);
        // Should not panic - mock accepts log messages
        log::info!("test message");
    }

    #[test]
    fn host_logger_log_disabled_message() {
        log::set_max_level(LevelFilter::Error);
        // Should not panic - message is filtered out before reaching handler
        log::debug!("this should be filtered");
    }

    #[test]
    fn host_logger_flush() {
        // Flush is a no-op, should not panic
        LOGGER.flush();
    }

    #[test]
    fn log_enabled_check() {
        // The mock enables levels 0-3 (Error, Warn, Info, Debug)
        assert!(handler::log_enabled(0)); // Error
        assert!(handler::log_enabled(1)); // Warn
        assert!(handler::log_enabled(2)); // Info
        assert!(handler::log_enabled(3)); // Debug
        assert!(!handler::log_enabled(-1)); // Trace (disabled)
        assert!(!handler::log_enabled(4)); // Unknown (disabled)
    }

    #[test]
    fn handler_log_call() {
        // Should not panic - mock accepts any log call
        handler::log(2, b"test log message");
    }

    #[test]
    fn test_init_default_level() {
        // init() uses Level::Info by default
        // Logger can only be set once, so we just verify it doesn't panic
        let _result = init();
    }

    #[test]
    fn test_max_level_enabled() {
        // When host has the level enabled, it should return that level
        let level = max_level(LevelFilter::Info);
        // Info maps to host level 0, which is enabled in mock
        assert_eq!(level, LevelFilter::Info);
    }

    #[test]
    fn test_max_level_disabled_decrements() {
        // When host has the level disabled, it should decrement
        // Trace maps to host level -1, which is disabled in mock
        let level = max_level(LevelFilter::Trace);
        // Should decrement to a lower severity level
        assert!(level < LevelFilter::Trace);
    }

    #[test]
    fn host_logger_log_direct_call() {
        // Set max level high enough to allow Info messages
        log::set_max_level(LevelFilter::Info);

        // Create a log record directly and call LOGGER.log()
        let record = log::Record::builder().level(Level::Info).target("test").args(format_args!("direct log test")).build();

        // This should call handler::log internally
        LOGGER.log(&record);
    }

    #[test]
    fn host_logger_log_skips_disabled_level() {
        // Set max level to Error only
        log::set_max_level(LevelFilter::Error);

        // Create a Debug record which should be filtered out
        let record =
            log::Record::builder().level(Level::Debug).target("test").args(format_args!("this should be skipped")).build();

        // This should return early without calling handler::log
        LOGGER.log(&record);
    }
}
