use std::fmt::Display;

pub struct Logger {
    logging: Vec<Log>,
}

#[derive(PartialEq)]
pub enum Log {
    Info,
    Warn,
    Debug,
    Error,
}

impl Default for Logger {
    fn default() -> Self {
        Self { logging: vec![Log::Info, Log::Warn, Log::Error] }
    }
}

impl Logger {
    pub fn info<T: Display>(&self, msg: T) {
        if self.logging.contains(&Log::Info) {
            println!("{msg}");
        }
    }
    pub fn warn<T: Display>(&self, msg: T) {
        if self.logging.contains(&Log::Warn) {
            println!("{msg}");
        }
    }
    pub fn debug<T: Display>(&self, msg: T) {
        if self.logging.contains(&Log::Debug) {
            println!("{msg}");
        }
    }
    pub fn error<T: Display>(&self, msg: T) {
        if self.logging.contains(&Log::Error) {
            eprintln!("{msg}");
        }
    }
}
