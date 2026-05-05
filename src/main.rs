use clap::{Parser, ValueEnum};
use std::collections::HashMap;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "mantylog", version, about = "log analyzer")]
struct Args {
    /// 입력 로그 파일 경로
    #[arg(short, long, default_value = "sample.log")]
    input: String,

    /// 특정 레벨만 출력 (지정 안하면 전부)
    #[arg(short, long, value_enum)]
    filter: Option<LevelArg>,
}

#[derive(Debug, Clone, ValueEnum)]
enum LevelArg {
    Error,
    Warn,
    Info,
    Unknown,
}

impl LevelArg {
    fn to_log_level(&self) -> LogLevel {
        match self {
            LevelArg::Error => LogLevel::Error,
            LevelArg::Warn => LogLevel::Warn,
            LevelArg::Info => LogLevel::Info,
            LevelArg::Unknown => LogLevel::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Unknown,
}

impl LogLevel {
    fn label(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Unknown => "UNKNOWN",
        }
    }
}

fn parse_log_level(line: &str) -> LogLevel {
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

fn first_token(line: &str) -> Option<&str> {
    line.split_whitespace().next()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)?;

    if let Some(first_line) = contents.lines().next() {
        match first_token(first_line) {
            Some(token) => println!("first token: {}", token),
            None => println!("empty line"),
        }
    }

    let mut counts: HashMap<LogLevel, u32> = HashMap::new();

    for line in contents.lines() {
        let level = parse_log_level(line);

        *counts.entry(level.clone()).or_insert(0) += 1;

        let should_print = match &args.filter {
            Some(arg) => arg.to_log_level() == level,
            None => true,
        };

        if should_print {
            print_line(level.label(), line);
        }
    }

    if let Some(&n) = counts.get(&LogLevel::Unknown) {
        println!("⚠️  {} 라인이 분류 불가", n);
    }

    println!("--- summary ---");
    println!(
        "ERROR: {}, WARN: {}, INFO: {}, UNKNOWN: {}",
        counts.get(&LogLevel::Error).unwrap_or(&0),
        counts.get(&LogLevel::Warn).unwrap_or(&0),
        counts.get(&LogLevel::Info).unwrap_or(&0),
        counts.get(&LogLevel::Unknown).unwrap_or(&0),
    );

    Ok(())
}

fn print_line(level: &str, line: &str) {
    println!("[{:<7}] {}", level, line);
}
