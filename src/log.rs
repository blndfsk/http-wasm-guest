use crate::host::handler;

pub enum Level {
    Debug = -1,
    Info = 0,
    Warn = 1,
    Error = 2,
    None = 3,
}
pub fn enabled(level: Level) -> bool {
    handler::log_enabled(level as i32)
}
///writes a message to the host's logs at the given level.
pub fn write(level: Level, message: &str) {
    if message.is_empty() {
        return;
    }
    handler::log(level as i32, message.as_bytes());
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Debug, $($arg)+))
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Info, $($arg)+))
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::log::Level::Warn, $($arg)+))
}
#[macro_export]
macro_rules! log {
    // log!(Level::Info, "a log event {}", param1)
    ($lvl:expr, $($arg:tt)+) => { $crate::log::write($lvl, format!($($arg)+).as_str()); };
}
