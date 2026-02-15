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
        handler::log(
            map_to_host(record.metadata().level()),
            format!("{}", record.args()).as_bytes(),
        );
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
    if handler::log_enabled(level.to_level().map_or_else(|| 3, map_to_host)) {
        level
    } else {
        level.decrement_severity()
    }
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
        let sut = init_with_level(Level::Info);
        assert!(sut.is_ok());
        assert_eq!(log::max_level(), LevelFilter::Info);
    }
}
