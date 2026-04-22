use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::Write;

use crate::{host, memory};

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
            memory::with_buffer(|buf| {
                let written = format_log_message(buf, record.args());
                host::log::write(host_level(record.metadata()), buf.as_subslice(written));
            });
        }
    }

    fn flush(&self) {}
}

/// Formats the log message into the provided buffer, applying truncation if needed.
/// Returns the number of bytes written.
fn format_log_message(buf: &mut memory::Buffer, args: &std::fmt::Arguments) -> usize {
    let capacity = buf.capacity();
    let mut slice = buf.as_mut_slice();
    match write!(slice, "{}", args) {
        Ok(()) => capacity - slice.len(),
        Err(_) => {
            let start = capacity - TRUNC_MARKER.len();
            let slice = buf.as_mut_slice();
            slice[start..].copy_from_slice(TRUNC_MARKER);
            buf.capacity()
        }
    }
}

impl HostLogger {
    /// Initialize the host-backed logger with the default Info level.
    ///
    /// This is a convenience function for [`init_with_level`] using `Level::Info`.
    #[inline]
    pub fn init() -> Result<(), SetLoggerError> {
        HostLogger::init_with_level(Level::Info)
    }

    /// Initialize the host-backed logger with a specific maximum level.
    ///
    /// This registers a HostLogger implementation for forwarding log records to the http-wasm host.
    #[inline]
    pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
        log::set_max_level(max_level(level.to_level_filter()));
        log::set_logger(&LOGGER)
    }
}

/// Determine the max_log_level as configured by the host.
/// If the log-level is more restrictive on the host than the plugin tries to configure,
/// the level is decremented until an enabled level is found or Off is reached.
fn max_level(level_filter: LevelFilter) -> LevelFilter {
    max_level_with(level_filter, |level| host::log::enabled(map_to_host(level)))
}

/// Core max-level selection logic parameterized by a host enable-check.
fn max_level_with(mut level_filter: LevelFilter, is_enabled: impl Fn(Level) -> bool) -> LevelFilter {
    while let Some(level) = level_filter.to_level() {
        if is_enabled(level) {
            return level_filter;
        }
        level_filter = level_filter.decrement_severity();
    }
    LevelFilter::Off
}

/// Map a Rust `log::Level` to the host severity code.
///
/// per spec: debug -1, info 0, warn 1, error 2, none 3
/// traefik logs with trace -2, debug -1, info 0, warn 1, error 2, (fatal 3)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_with_level() {
        // Logger can only be set once globally, so we just verify it doesn't panic
        // and returns a result (either Ok or Err if already set)
        let _result = HostLogger::init();
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
    fn test_log_truncation_marker() {
        // Compose a message that will overflow the buffer
        let long_msg = "A".repeat(3000);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &format_args!("{}", long_msg));
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), buf.capacity(), "Truncated log should fill the buffer");
            assert!(slice.ends_with(TRUNC_MARKER), "Log message should end with truncation marker");
        });
    }

    #[test]
    fn test_format_log_message() {
        let msg = "Test";
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &format_args!("{}", msg));
            assert_eq!(written, msg.len(), "message should not be truncated");
            assert_eq!(buf.as_subslice(written), msg.as_bytes());
        });
    }

    #[test]
    fn test_format_log_message_limit() {
        let msg = "A".repeat(2048);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &format_args!("{}", msg));
            assert_eq!(written, msg.len(), "message should not be truncated");
            assert_eq!(buf.as_subslice(written), msg.as_bytes());
        });
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
    fn host_logger_flush() {
        // Flush is a no-op, should not panic
        LOGGER.flush();
    }

    #[test]
    fn test_max_level_enabled() {
        // When host has the level enabled, it should return that level
        let level = max_level(LevelFilter::Info);
        // Info maps to host level 0, which is enabled in mock
        assert_eq!(level, LevelFilter::Info);
    }

    #[test]
    fn test_max_level_off_stays_off() {
        // Off is the terminal state for level reduction and should return immediately.
        assert_eq!(max_level(LevelFilter::Off), LevelFilter::Off);
    }

    #[test]
    fn test_max_level_returns_off_when_host_disables_everything() {
        let level = max_level_with(LevelFilter::Trace, |_| false);
        assert_eq!(level, LevelFilter::Off);
    }

    #[test]
    fn test_max_level_stops_at_first_enabled_level() {
        let level = max_level_with(LevelFilter::Trace, |level| matches!(level, Level::Warn | Level::Error));
        assert_eq!(level, LevelFilter::Warn);
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
        // Set max level to Warn
        log::set_max_level(LevelFilter::Warn);
        // Call max_level with disabled level, which should decrement to Warn
        let result = max_level(LevelFilter::Warn);
        assert_eq!(result, LevelFilter::Warn, "max_level should decrement to Warn when only Warn is enabled on host");
    }
}
