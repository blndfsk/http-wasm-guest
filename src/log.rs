use crate::host::handler;

pub enum Level {
    Debug = -1,
    Info = 0,
    Warn = 1,
    Error = 2,
    None = 3,
}
pub fn log_enabled(level: Level) -> bool {
    handler::log_enabled(level as i32)
}
/**
log adds a UTF-8 encoded message to the host's logs at the given level.
*/
pub fn log(level: Level, message: &str) {
    if message.is_empty() {
        return;
    }
    handler::log(level as i32, message.as_bytes());
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::__log!($crate::log::Level::Debug, $($arg)+))
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::__log!($crate::log::Level::Info, $($arg)+))
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::__log!($crate::log::Level::Warn, $($arg)+))
}
#[macro_export]
macro_rules! __log {
    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => {{
        $crate::log::log($lvl, format!($($arg)+).as_str());
    }};
}
