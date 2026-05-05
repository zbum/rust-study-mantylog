mod cli;
mod level;
mod parse;

use clap::Parser;
use std::collections::HashMap;
use std::fs;

use cli::Args;
use level::{parse_log_level, LogLevel};
use parse::first_token;

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
            Some(arg) => LogLevel::from(arg.clone()) == level,
            None => true,
        };

        if should_print {
            print_line(&level, line);
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

fn print_line(level: &LogLevel, line: &str) {
    println!("[{:<7}] {}", level, line);
}
