use serde::Serialize;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
#[serde(rename_all = "UPPERCASE")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_error() {
        assert_eq!(parse_log_level("foo ERROR bar"), LogLevel::Error);
    }

    #[test]
    fn parses_warn() {
        assert_eq!(parse_log_level("WARN message"), LogLevel::Warn);
    }

    #[test]
    fn parses_info() {
        assert_eq!(parse_log_level("INFO ok"), LogLevel::Info);
    }

    #[test]
    fn unknown_for_unrecognized() {
        assert_eq!(parse_log_level("no recognizable level"), LogLevel::Unknown);
    }

    #[test]
    fn priority_error_over_warn() {
        // 한 라인에 두 키워드 있으면 우선순위 ERROR > WARN > INFO
        assert_eq!(parse_log_level("WARN with ERROR mixed"), LogLevel::Error);
    }

    #[test]
    fn level_arg_converts_to_log_level() {
        use crate::cli::LevelArg;
        assert_eq!(LogLevel::from(LevelArg::Error), LogLevel::Error);
        assert_eq!(LogLevel::from(LevelArg::Warn), LogLevel::Warn);
        assert_eq!(LogLevel::from(LevelArg::Info), LogLevel::Info);
        assert_eq!(LogLevel::from(LevelArg::Unknown), LogLevel::Unknown);
    }

    #[test]
    fn display_outputs_uppercase_label() {
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
        assert_eq!(format!("{}", LogLevel::Warn), "WARN");
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Unknown), "UNKNOWN");
    }

    #[test]
    fn display_respects_width_padding() {
        // f.pad()가 Formatter의 폭/정렬을 존중하는지 — Step 7의 회귀 방지
        assert_eq!(format!("[{:<7}]", LogLevel::Info), "[INFO   ]");
        assert_eq!(format!("[{:<7}]", LogLevel::Error), "[ERROR  ]");
        assert_eq!(format!("[{:<7}]", LogLevel::Unknown), "[UNKNOWN]");
    }

    #[test]
    fn serializes_to_uppercase_json() {
        // #[serde(rename_all = "UPPERCASE")] 검증
        assert_eq!(serde_json::to_string(&LogLevel::Error).unwrap(), "\"ERROR\"");
        assert_eq!(serde_json::to_string(&LogLevel::Warn).unwrap(), "\"WARN\"");
    }
}
