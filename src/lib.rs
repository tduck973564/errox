use colored::Colorize as Colourise;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::time;

lazy_static! {
    static ref DEFAULT_CONFIG: Config = Config { log_level: LogLevel::Trace, time: true };
    static ref START_TIME: u64 = current_time_secs();
    static ref CONFIG: Config = Config::new("errox_config.toml".to_string()).unwrap_or_else( |e| {
        error_format(LogLevel::Error, format!("{} {}", "Could not load config file, falling back to default.", e.to_string()));
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

fn calculate_precedence(log_level: &LogLevel) -> bool {
    if log_level < &CONFIG.log_level {
        return false;
    }
    true
}

fn error_format(log_level: LogLevel, message: String) -> Option<String> {
    let mut out = String::new();
    if !calculate_precedence(&log_level) {
        return None;
    }
    if CONFIG.time {
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

pub fn error(message: &str) {
    error_format(LogLevel::Error, message.to_string());
}
pub fn warning(message: &str) {
    error_format(LogLevel::Warning, message.to_string());
}
pub fn info(message: &str) {
    error_format(LogLevel::Info, message.to_string());
}
pub fn debug(message: &str) {
    error_format(LogLevel::Debug, message.to_string());
}
pub fn trace(message: &str) {
    error_format(LogLevel::Trace, message.to_string());
}

pub trait Errox<T> {
    fn error(self) -> T;
    fn warning(self) -> T;
    fn info(self) -> T;
    fn debug(self) -> T;
    fn trace(self) -> T;
}

impl<T: Error> Errox<T> for T {
    fn error(self) -> T {
        error(&self.to_string());
        self
    }
    fn warning(self) -> T {
        warning(&self.to_string());
        self
    }
    fn info(self) -> T {
        info(&self.to_string());
        self
    }
    fn debug(self) -> T {
        debug(&self.to_string());
        self
    }
    fn trace(self) -> T {
        debug(&self.to_string());
        self
    }
}
