use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::Write;

use crate::{
    host,
    memory::{self, Buffer},
};

static LOGGER: HostLogger = HostLogger;
const TRUNC_MARKER: &[u8] = b"... [truncated]";

/// Logger implementation that forwards records to the host.
///
/// This integrates the Rust `log` crate with the http-wasm guest runtime's logging system.
/// It provides logging for plugin authors via standard macros (`log::info!`, `log::warn!`, etc.).
pub struct HostLogger;

impl Log for HostLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let (buf, written) = format_log_message(record.args());
            host::log::write(host_level(record.metadata()), buf.as_subslice(written));
        }
    }

    fn flush(&self) {}
}

/// Formats the log message into the static buffer, applying truncation if needed.
/// Returns a reference to the buffer and the number of bytes written.
fn format_log_message(args: &std::fmt::Arguments) -> (&'static mut Buffer, usize) {
    // SAFETY: WASM guest is single-threaded.
    let buf = memory::buffer();
    let capacity = buf.capacity();

    let mut slice = buf.as_mut_slice();
    let result = write!(slice, "{}", args);
    let mut written = capacity - slice.len();
    // If the message could be written, change the last bytes to marker
    if result.is_err() {
        let start = capacity - TRUNC_MARKER.len();
        let slice = buf.as_mut_slice();
        slice[start..].copy_from_slice(TRUNC_MARKER);
        written = buf.capacity();
    }

    (buf, written)
}

impl HostLogger {
    /// Initialize the host-backed logger with a specific maximum level.
    ///
    /// This registers a HostLogger implementation for forwarding log records to the http-wasm host.
    pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
        set_global_logger(level)
    }

    /// Initialize the host-backed logger with the default Info level.
    ///
    /// This is a convenience wrapper for [`init_with_level`] using `Level::Info`.
    pub fn init() -> Result<(), SetLoggerError> {
        set_global_logger(Level::Info)
    }
}
fn set_global_logger(level: Level) -> Result<(), SetLoggerError> {
    log::set_max_level(max_level(level.to_level_filter()));
    log::set_logger(&LOGGER)
}
/// Determine the max_log_level as configured by the host
/// If the log-level is more restrictive on the host as the plugin tries to configure,
/// the level is decremented until an enabled level is found.
fn max_level(mut level_filter: LevelFilter) -> LevelFilter {
    loop {
        if host::log::enabled(level_filter.to_level().map_or_else(|| -3, map_to_host)) {
            return level_filter;
        } else {
            level_filter = level_filter.decrement_severity();
        }
    }
}
/// Map a Rust `log::Level` to the host severity code.
///
/// The mapping is defined by `LVL` and must stay consistent with the host.
fn map_to_host(level: Level) -> i32 {
    match level {
        Level::Error => 2,
        Level::Warn => 1,
        Level::Info => 0,
        Level::Debug => -1,
        Level::Trace => -2,
    }
}
fn host_level(md: &Metadata) -> i32 {
    map_to_host(md.level())
    // match md.level() {
    //     Level::Error => match md.target() {
    //         "panic" => 4,
    //         "fatal" => 3,
    //         _ => map_to_host(md.level()),
    //     },
    //     _ => map_to_host(md.level()),
    // }
}

#[cfg(test)]
mod tests {
    use log::MetadataBuilder;

    use super::*;

    #[test]
    fn test_init_with_level() {
        // Logger can only be set once globally, so we just verify it doesn't panic
        // and returns a result (either Ok or Err if already set)
        let _result = HostLogger::init_with_level(Level::Info);
        // If this is the first init, max_level should be Info
        // If logger was already set, this is still valid
    }

    #[test]
    fn map_level_to_host() {
        assert_eq!(map_to_host(Level::Error), 2);
        assert_eq!(map_to_host(Level::Warn), 1);
        assert_eq!(map_to_host(Level::Info), 0);
        assert_eq!(map_to_host(Level::Debug), -1);
        assert_eq!(map_to_host(Level::Trace), -2);
    }

    #[test]
    fn map_host_level() {
        assert_eq!(host_level(&MetadataBuilder::new().target("fatal").build()), 0);
        assert_eq!(host_level(&MetadataBuilder::new().level(Level::Error).target("").build()), 2);
        assert_eq!(host_level(&MetadataBuilder::new().level(Level::Error).target("fatal").build()), 2);
        assert_eq!(host_level(&MetadataBuilder::new().level(Level::Error).target("panic").build()), 2);
    }
    #[test]
    fn test_log_truncation_marker() {
        // Compose a message that will overflow the buffer
        let long_msg = "A".repeat(3000);
        let (buf, written) = super::format_log_message(&format_args!("{}", long_msg));
        let slice = buf.as_subslice(written);
        assert_eq!(slice.len(), buf.capacity(), "Truncated log should fill the buffer");
        assert!(slice.ends_with(TRUNC_MARKER), "Log message should end with truncation marker");
    }

    #[test]
    fn test_format_log_message() {
        // Compose a message that will overflow the buffer
        let msg = "Test";
        let (buf, written) = super::format_log_message(&format_args!("{}", msg));
        assert_eq!(written, msg.len(), "message should not be truncated");
        assert_eq!(buf.as_subslice(written), msg.as_bytes());
    }

    #[test]
    fn test_format_log_message_limit() {
        // Compose a message that will overflow the buffer
        let msg = "A".repeat(2048);
        let (buf, written) = super::format_log_message(&format_args!("{}", msg));
        assert_eq!(written, msg.len(), "message should not be truncated");
        assert_eq!(buf.as_subslice(written), msg.as_bytes());
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
    fn test_init_default_level() {
        // init() uses Level::Info by default
        // Logger can only be set once, so we just verify it doesn't panic
        let _result = HostLogger::init();
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

    #[test]
    fn test_max_level_decrement_until_enabled() {
        // Start at Trace
        let level = LevelFilter::Trace;

        // Call max_level, which should decrement to Info
        let result = max_level(level);

        assert_eq!(result, LevelFilter::Info, "max_level should decrement to Warn when only Warn is enabled on host");
    }
}
