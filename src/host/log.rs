pub enum Level {
    Debug = -1,
    Info = 0,
    Warn = 1,
    Error = 2,
    None = 3,
}
use super::handler;

///writes a message to the host's logs at the given level.
pub fn write(level: Level, message: &str) {
    if message.is_empty() {
        return;
    }
    handler::log(level as i32, message.as_bytes());
}
pub fn enabled(level: Level) -> bool {
    handler::log_enabled(level as i32)
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log!($crate::host::log::Level::Debug, $($arg)+))
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::host::log::Level::Info, $($arg)+))
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::host::log::Level::Warn, $($arg)+))
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::log!($crate::host::log::Level::Error, $($arg)+))
}
#[macro_export]
macro_rules! log {
    // log!(Level::Info, "a log event {}", param1)
    ($lvl:expr, $($arg:tt)+) => { $crate::host::log::write($lvl, format!($($arg)+).as_str()); };
}
