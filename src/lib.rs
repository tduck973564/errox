/*
 * Copyright (c) 2021-2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `errox` project, which is licenced under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html.
 */

use doc_comment::doc_comment;
doc_comment!(include_str!("../README.md"));

use colored::Colorize as Colourise;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::time;
use std::fmt::Display;

#[cfg(test)]
mod tests;

lazy_static! {
    static ref DEFAULT_CONFIG: Config = Config {
        log_level: LogLevel::Trace,
        time: true
    };
    static ref START_TIME: u64 = current_time_secs();
    static ref CONFIG: Config = Config::new("errox_config.toml".to_string()).unwrap_or_else(|e| {
        error_format(
            LogLevel::Error,
            format!(
                "{} {}",
                "Could not load config file, falling back to default.",
                e.to_string()
            ),
            &DEFAULT_CONFIG,
        );
        (*DEFAULT_CONFIG).clone()
    });
}

#[derive(Deserialize, Clone)]
struct Config {
    time: bool,
    log_level: LogLevel,
}

#[derive(PartialEq, PartialOrd, Deserialize, Clone)]
#[serde(tag = "type")]
enum LogLevel {
    Error = 4,
    Warning = 3,
    Info = 2,
    Debug = 1,
    Trace = 0,
}

impl Config {
    pub fn new(path: String) -> Result<Config, Box<dyn Error>> {
        let raw_file = fs::read_to_string(path)?;
        Ok(toml::from_str(&raw_file)?)
    }
}

fn current_time_secs() -> u64 {
    let start = time::SystemTime::now();
    start
        .duration_since(time::UNIX_EPOCH)
        .expect("What")
        .as_secs()
}

fn secs_elapsed() -> u64 {
    current_time_secs() - *START_TIME
}

fn calculate_precedence(log_level: &LogLevel, config: &Config) -> bool {
    if log_level < &config.log_level {
        return false;
    }
    true
}

fn error_format(log_level: LogLevel, message: String, config: &Config) -> Option<String> {
    let mut out = String::new();
    if !calculate_precedence(&log_level, config) {
        return None;
    }
    if config.time {
        out += &format!("[{}] ", secs_elapsed().to_string().yellow());
    }
    out = format!(
        "{}{}",
        out,
        match log_level {
            LogLevel::Error => format!("{}{} ", "error".red().bold(), ":".bold()),
            LogLevel::Warning => format!("{}{} ", "warning".yellow().bold(), ":".bold()),
            LogLevel::Info => format!("{}{} ", "info".blue().bold(), ":".bold()),
            LogLevel::Debug => format!("{}{} ", "debug".bold(), ":".bold()),
            LogLevel::Trace => format!("{}{} ", "trace".bold(), ":".bold()),
        }
    );
    out += &message;
    Some(out)
}

/// Prints an error message to stderr.
pub fn error(message: &str) {
    if let Some(x) = error_format(LogLevel::Error, message.to_string(), &CONFIG) {
        eprintln!("{}", x);
    }
}

/// Prints a warning to stderr, if the log level is set low enough.
pub fn warning(message: &str) {
    if let Some(x) = error_format(LogLevel::Warning, message.to_string(), &CONFIG) {
        eprintln!("{}", x);
    }
}

/// Prints an information message to stderr, if the log level is set low enough.
pub fn info(message: &str) {
    if let Some(x) = error_format(LogLevel::Info, message.to_string(), &CONFIG) {
        eprintln!("{}", x);
    }
}

/// Prints a debug message to stderr, if the log level is set low enough.
pub fn debug(message: &str) {
    if let Some(x) = error_format(LogLevel::Debug, message.to_string(), &CONFIG) {
        eprintln!("{}", x);
    }
}

/// Prints a trace message to stderr, if the log level is set low enough.
pub fn trace(message: &str) {
    if let Some(x) = error_format(LogLevel::Trace, message.to_string(), &CONFIG) {
        eprintln!("{}", x);
    }
}

/// Trait with methods error(), warning(), info(), debug() and trace() over Result<T, impl Display> (Display is blanket implemented by Error, implement Error over your types). The error message is self.to_string(), from the Display trait implementation. This trait is much preferable to using the function counterparts.
pub trait Errox<T> {
    fn error(self) -> T;
    fn warning(self) -> T;
    fn info(self) -> T;
    fn debug(self) -> T;
    fn trace(self) -> T;
}

impl<T, E: Display> Errox<Result<T, E>> for Result<T, E> {
    fn error(self) -> Result<T, E> {
        if let Err(x) = &self {
            error(&x.to_string());
        }
        self
    }
    fn warning(self) -> Result<T, E> {
        if let Err(x) = &self {
            warning(&x.to_string());
        }
        self
    }
    fn info(self) -> Result<T, E> {
        if let Err(x) = &self {
            info(&x.to_string());
        }
        self
    }
    fn debug(self) -> Result<T, E> {
        if let Err(x) = &self {
            debug(&x.to_string());
        }
        self
    }
    fn trace(self) -> Result<T, E> {
        if let Err(x) = &self {
            trace(&x.to_string());
        }
        self
    }
}
