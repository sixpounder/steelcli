use std::{sync::Mutex};
use colored::*;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    Verbose = 2,
    Normal = 1,
    Muted = 0
}
pub struct Log {
    level: Mutex<LogLevel>
}

const LOG_SIGN: char = '\u{1F535}'; // Blue circle
const VERBOSE_SIGN: char = '\u{1F7E3}'; // Purple circle
const SUCCESS_SIGN: char = '\u{2714}'; // Check mark
const ERROR_SIGN: char = '\u{274C}'; // Cross mark
const WARN_SIGN: char = '\u{26A0}'; // Cross mark

impl Log {
    pub fn new() -> Self {
        Self {
            level: Mutex::new(LogLevel::Normal)
        }
    }

    pub fn set_level(&self, level: LogLevel) {
        let mut w = self.level.lock().unwrap();
        *w = level;
    }

    pub fn log_wrap<F>(&self, f: F) where F: FnOnce() {
        if *self.level.lock().unwrap() >= LogLevel::Normal {
            f()
        }
    }

    pub fn verbose_wrap<F>(&self, f: F) where F: FnOnce() {
        if *self.level.lock().unwrap() >= LogLevel::Verbose {
            f()
        }
    }

    pub fn log(&self, msg: &str) {
        if *self.level.lock().unwrap() >= LogLevel::Normal {
            println!("{} {}", LOG_SIGN, msg);
        }
    }

    pub fn verbose(&self, msg: &str) {
        if *self.level.lock().unwrap() >= LogLevel::Verbose {
            println!("{} {}", VERBOSE_SIGN, msg);
        }
    }

    pub fn success(&self, msg: &str) {
        println!("{} {}", SUCCESS_SIGN, msg.green());
    }

    pub fn warn(&self, msg: &str) {
        println!("{} {}", WARN_SIGN, msg.yellow());
    }

    pub fn error(&self, msg: &str) {
        println!("{} {}", ERROR_SIGN, msg.red());
    }
}
