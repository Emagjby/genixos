use core::fmt::Arguments;

use crate::cprintln;

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn log(level: LogLevel, args: Arguments) {
    match level {
        LogLevel::Debug => cprintln!(DarkGray; "[DEBUG] {}", args),
        LogLevel::Info => cprintln!(LightCyan; "[INFO] {}", args),
        LogLevel::Warn => cprintln!(Yellow; "[WARN] {}", args),
        LogLevel::Error => cprintln!(LightRed; "[ERROR] {}", args),
    }
}
