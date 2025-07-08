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

#[inline]
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

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
