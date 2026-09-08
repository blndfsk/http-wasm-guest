use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::io::Write;

#[cfg(not(test))]
use crate::memory::SyncCell;
use crate::{host, memory};

static LOGGER: HostLogger = HostLogger;

/// Configuration for [`HostLogger`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostLoggerConfig {
    /// Maximum log level to enable in Rust. Defaults to `Level::Info`.
    pub level: Level,
    /// Maximum formatted message length (in bytes) before truncation. Defaults to `2048`.
    pub max_message_len: usize,
    /// Marker used to signal truncation. Defaults to `b"...`.
    pub trunc_marker: &'static [u8],
    /// Whether to include the target in the log message. Defaults to `false`.
    pub with_target: bool,
}

impl HostLoggerConfig {
    const DEFAULT: Self = Self { level: Level::Info, max_message_len: 2048, trunc_marker: b"...", with_target: false };
}

impl Default for HostLoggerConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(not(test))]
static LOGGER_CONFIG: SyncCell<HostLoggerConfig> = SyncCell::new(HostLoggerConfig::DEFAULT);

#[cfg(test)]
thread_local! {
    static LOGGER_CONFIG: std::cell::UnsafeCell<HostLoggerConfig> =
        const { std::cell::UnsafeCell::new(HostLoggerConfig::DEFAULT) };
}

#[cfg(not(test))]
fn with_logger_config<R>(f: impl FnOnce(&mut HostLoggerConfig) -> R) -> R {
    // SAFETY: WASM guest is single-threaded.
    let cfg = unsafe { &mut *LOGGER_CONFIG.get() };
    f(cfg)
}

#[cfg(test)]
fn with_logger_config<R>(f: impl FnOnce(&mut HostLoggerConfig) -> R) -> R {
    LOGGER_CONFIG.with(|cell| {
        // SAFETY: thread-local; no cross-thread aliasing.
        let cfg = unsafe { &mut *cell.get() };
        f(cfg)
    })
}

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
            with_logger_config(|config| {
                memory::with_buffer(|buf| {
                    let written = format_log_message(buf, record, config);
                    host::log::write(host_level(record.metadata()), buf.as_subslice(written));
                });
            });
        }
    }

    fn flush(&self) {}
}

/// Formats the log message into the provided buffer, applying truncation if needed.
/// Returns the number of bytes written.
fn format_log_message(buf: &mut memory::Buffer, record: &Record, config: &HostLoggerConfig) -> usize {
    let limit = config.max_message_len.min(buf.capacity());
    if limit == 0 {
        return 0;
    }

    let written = {
        let mut slice = &mut buf.as_mut_slice()[..limit];
        if config.with_target && record.target().len() + 2 <= limit {
            write!(slice, "{}: {}", record.target(), record.args())
        } else {
            write!(slice, "{}", record.args())
        }
        .map(|()| limit - slice.len())
    };

    match written {
        Ok(written) => written,
        Err(_) => {
            if limit >= config.trunc_marker.len() {
                let start = limit - config.trunc_marker.len();
                let slice = &mut buf.as_mut_slice()[..limit];
                slice[start..].copy_from_slice(config.trunc_marker);
            }
            limit
        }
    }
}

impl HostLogger {
    /// Initialize the host-backed logger with default configuration.
    #[inline]
    pub fn init() -> Result<(), SetLoggerError> {
        HostLogger::init_with_config(HostLoggerConfig::default())
    }

    /// Initialize the host-backed logger with a specific maximum level.
    #[inline]
    pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
        HostLogger::init_with_config(HostLoggerConfig { level, ..HostLoggerConfig::default() })
    }

    /// Initialize the host-backed logger with full configuration.
    #[inline]
    pub fn init_with_config(config: HostLoggerConfig) -> Result<(), SetLoggerError> {
        with_logger_config(|cfg| *cfg = config);
        log::set_max_level(max_level(config.level.to_level_filter()));
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
    use log::RecordBuilder;

    use super::*;

    #[test]
    fn test_init_with_config() {
        // Logger can only be set once globally, so we just verify it doesn't panic
        // and returns a result (either Ok or Err if already set)
        let _result = HostLogger::init_with_config(HostLoggerConfig {
            level: Level::Debug,
            max_message_len: 1024,
            trunc_marker: b"test",
            with_target: true,
        });
        // If this is the first init, max_level should be Info
        // If logger was already set, this is still valid
        with_logger_config(|config| {
            assert_eq!(config.level, Level::Debug);
            assert_eq!(config.max_message_len, 1024);
            assert_eq!(config.trunc_marker, b"test");
            assert!(config.with_target);
        })
    }

    #[test]
    fn host_logger_config_default_values() {
        let config = HostLoggerConfig::default();
        assert_eq!(config.level, Level::Info);
        assert_eq!(config.max_message_len, 2048);
        assert_eq!(config.trunc_marker, b"...");
        assert!(!config.with_target);
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
    fn test_format_log_message_respects_configured_max_len() {
        let config = HostLoggerConfig { max_message_len: 64, ..HostLoggerConfig::default() };
        let msg = "A".repeat(100);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should respect configured max length");
        });
    }

    #[test]
    fn test_log_truncation_marker() {
        let config = HostLoggerConfig { max_message_len: 10, ..HostLoggerConfig::default() };
        let msg = "A".repeat(30);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should fill the configured max length");
            assert!(slice.ends_with(config.trunc_marker), "Log message should end with truncation marker");
        });
    }

    #[test]
    fn test_log_truncation_tiny_limit_without_marker() {
        let config = HostLoggerConfig { max_message_len: 2, ..HostLoggerConfig::default() };
        let msg = "Z".repeat(30);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should stay within configured max length");
            assert_eq!(slice, b"ZZ", "Tiny limits should truncate without appending marker");
        });
    }
    #[test]
    fn test_log_truncation_zero_limit() {
        let config = HostLoggerConfig { max_message_len: 0, ..HostLoggerConfig::default() };
        let msg = "Z".repeat(30);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should stay within configured max length");
            assert_eq!(slice, b"", "Zero limit should truncate to empty");
        });
    }

    #[test]
    fn test_log_truncation_without_marker() {
        let config = HostLoggerConfig { max_message_len: 20, trunc_marker: b"", ..HostLoggerConfig::default() };
        let msg = "Z".repeat(30);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should stay within configured max length");
            assert!(slice.ends_with(b"ZZZ"));
        });
    }
    #[test]
    fn test_log_overflow() {
        let config = HostLoggerConfig { max_message_len: 3000, trunc_marker: b"", ..HostLoggerConfig::default() };
        let msg = "Z".repeat(3000);
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            let slice = buf.as_subslice(written);
            assert_eq!(slice.len(), buf.capacity(), "log should stay within max length of buf");
            assert!(slice.ends_with(b"ZZZ"));
        });
    }

    #[test]
    fn test_format_log_message() {
        let msg = "Test";
        let config = HostLoggerConfig::default();
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            assert_eq!(written, msg.len(), "message should not be truncated");
            assert_eq!(buf.as_subslice(written), msg.as_bytes());
        });
    }

    #[test]
    fn test_format_log_message_limit() {
        let msg = "A".repeat(2048);
        let config = HostLoggerConfig::default();
        memory::with_buffer(|buf| {
            let written = super::format_log_message(buf, &RecordBuilder::new().args(format_args!("{}", msg)).build(), &config);
            assert_eq!(written, msg.len(), "message should not be truncated");
            assert_eq!(buf.as_subslice(written), msg.as_bytes());
        });
    }

    #[test]
    fn test_format_log_message_with_target() {
        let msg = "Test";
        let config = HostLoggerConfig { with_target: true, ..Default::default() };
        memory::with_buffer(|buf| {
            let written = super::format_log_message(
                buf,
                &RecordBuilder::new().args(format_args!("{msg}")).target("target").build(),
                &config,
            );
            assert_eq!(buf.as_subslice(written), b"target: Test");
        });
    }
    #[test]
    fn test_format_log_message_with_target_truncates() {
        let msg = "A".repeat(30);
        let config = HostLoggerConfig { max_message_len: 20, with_target: true, ..Default::default() };
        memory::with_buffer(|buf| {
            let written = super::format_log_message(
                buf,
                &RecordBuilder::new().args(format_args!("{msg}")).target("target").build(),
                &config,
            );
            let slice = buf.as_subslice(written);
            assert_eq!(slice, b"target: AAAAAAAAA...");
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should respect configured max length");
        });
    }

    #[test]
    fn test_format_log_message_with_target_short_message_len() {
        let msg = "A".repeat(10);
        let config = HostLoggerConfig { max_message_len: 5, with_target: true, ..Default::default() };
        memory::with_buffer(|buf| {
            let written = super::format_log_message(
                buf,
                &RecordBuilder::new().args(format_args!("{msg}")).target("target").build(),
                &config,
            );
            let slice = buf.as_subslice(written);
            assert_eq!(slice, b"AA...");
            assert_eq!(slice.len(), config.max_message_len, "Truncated log should respect configured max length");
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
