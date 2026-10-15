use core::fmt::Arguments;

use crate::{cprintln, serialprintln};

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub fn log(level: LogLevel, args: Arguments) {
    match level {
        LogLevel::Debug => {
            cprintln!(DarkGray; "[DEBUG] {}", args);
            serialprintln!("[DEBUG] {}", args);
        }
        LogLevel::Info => {
            cprintln!(LightCyan; "[INFO] {}", args);
            serialprintln!("[INFO] {}", args);
        }
        LogLevel::Warn => {
            cprintln!(Yellow; "[WARN] {}", args);
            serialprintln!("[WARN] {}", args);
        }
        LogLevel::Error => {
            cprintln!(LightRed; "[ERROR] {}", args);
            serialprintln!("[ERROR] {}", args);
        }
    }
}
