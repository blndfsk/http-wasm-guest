//! Administrative utilities for configuring host features and logging.
//!
//! This module provides functions to enable host capabilities, read runtime
//! configuration, and route `log` crate output through the host.
use log::{Level, Log, Metadata, Record, SetLoggerError};

use crate::host::{Bytes, feature, handler};

/// Enables one or more host features and returns the host result code.
///
/// Combine feature flags with bitwise OR (e.g., `BufferRequest | BufferResponse`)
/// to enable multiple capabilities in a single call. The return value is the
/// host-provided status code.
pub fn enable(feature: feature::Feature) -> i32 {
    handler::enable_feature(feature.0)
}

/// Returns the raw configuration bytes provided by the host.
///
/// The host controls the configuration payload; interpret it according to your
/// plugin’s configuration format (for example, JSON or protobuf).
pub fn config() -> Bytes {
    Bytes::from(handler::get_config())
}

static LOGGER: HostLogger = HostLogger;
static LVL: [i32; 6] = [3, 2, 1, 0, -1, -1];

fn map(level: Level) -> i32 {
    LVL[level as usize]
}
struct HostLogger;

impl Log for HostLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level() && handler::log_enabled(map(metadata.level()))
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        handler::log(map(record.metadata().level()), format!("{}", record.args()).as_bytes());
    }

    fn flush(&self) {}
}

/// Initialize the host-backed logger with a specific maximum level.
///
/// After initialization, calls to the `log` crate are forwarded to the host
/// logger, subject to `level` and host-side filtering.
#[inline]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}
/// Initialize the host-backed logger with the default Info level.
///
/// This is a convenience wrapper around [`init_with_level`] using `Level::Info`.
#[inline]
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Info)
}
