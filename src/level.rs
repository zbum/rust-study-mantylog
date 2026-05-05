use std::fmt;

use crate::cli::LevelArg;

impl From<LevelArg> for LogLevel {
    fn from(arg: LevelArg) -> Self {
        match arg {
            LevelArg::Error => LogLevel::Error,
            LevelArg::Warn => LogLevel::Warn,
            LevelArg::Info => LogLevel::Info,
            LevelArg::Unknown => LogLevel::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Unknown,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Unknown => "UNKNOWN",
        };
        f.pad(s)
    }
}

pub fn parse_log_level(line: &str) -> LogLevel {
    if line.contains("ERROR") {
        LogLevel::Error
    } else if line.contains("WARN") {
        LogLevel::Warn
    } else if line.contains("INFO") {
        LogLevel::Info
    } else {
        LogLevel::Unknown
    }
}
