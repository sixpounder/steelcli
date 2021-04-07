use std::{sync::Mutex};

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    VERBOSE = 2,
    NORMAL = 1,
    MUTED = 0
}
pub struct Log {
    level: Mutex<LogLevel>
}

impl Log {
    pub fn new() -> Self {
        Self {
            level: Mutex::new(LogLevel::NORMAL)
        }
    }

    pub fn set_level(&self, level: LogLevel) {
        let mut w = self.level.lock().unwrap();
        *w = level;
    }

    pub fn log<F>(&self, f: F) where F: FnOnce() {
        if *self.level.lock().unwrap() >= LogLevel::NORMAL {
            f()
        }
    }

    pub fn verbose<F>(&self, f: F) where F: FnOnce() {
        if *self.level.lock().unwrap() >= LogLevel::VERBOSE {
            f()
        }
    }
}
