use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
#[command(name = "mantylog", version, about = "log analyzer")]
pub struct Args {
    /// 입력 로그 파일 경로
    #[arg(short, long, default_value = "sample.log")]
    pub input: String,

    /// 특정 레벨만 출력 (지정 안하면 전부)
    #[arg(short, long, value_enum)]
    pub filter: Option<LevelArg>,

    /// 출력 포맷
    #[arg(short = 'F', long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum LevelArg {
    Error,
    Warn,
    Info,
    Unknown,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OutputFormat::Text => "text",
            OutputFormat::Json => "json",
        };
        f.pad(s)
    }
}
