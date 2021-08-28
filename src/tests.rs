/*
 * Copyright (c) 2021-2021 Thomas Duckworth <tduck973564@gmail.com>.
 * This file is under the `errox` project, which is licenced under the GNU GPL v3.0 which you can read here: https://www.gnu.org/licenses/gpl-3.0.en.html.
 */

use super::*;
use std::fs::File;
#[test]
fn calculate_precedence() {
    assert!(super::calculate_precedence(
        &LogLevel::Trace,
        &Config {
            log_level: LogLevel::Trace,
            time: true
        }
    ));
    assert!(!super::calculate_precedence(
        &LogLevel::Trace,
        &Config {
            log_level: LogLevel::Error,
            time: true
        }
    ));
}

#[test]
fn error_format() {
    assert_eq!(
        super::error_format(
            LogLevel::Trace,
            "When the".to_string(),
            &Config {
                log_level: LogLevel::Error,
                time: true
            }
        ),
        None
    );
    if let None = super::error_format(
        LogLevel::Error,
        "When the".to_string(),
        &Config {
            log_level: LogLevel::Trace,
            time: true,
        },
    ) {
        panic!();
    }
}
