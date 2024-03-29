use chrono::Local;
use port::log::Log;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Error,
}

#[derive(Debug, Clone)]
pub struct Logger {
    id: String,
    level: Level,
}

impl Logger {
    pub fn new(id: String, level: Level) -> Self {
        Self { id, level }
    }
}

impl Log for Logger {
    fn debug(&self, s: &str) {
        if self.level <= Level::Debug {
            println!(
                " \x1b[47m\x1b[45m DEBUG \x1b[m {} | {} | {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                self.id,
                s,
            );
        }
    }

    fn info(&self, s: &str) {
        if self.level <= Level::Info {
            println!(
                " \x1b[47m\x1b[44m INFO  \x1b[m {} | {} | {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                self.id,
                s,
            );
        }
    }

    fn error(&self, s: &str) {
        if self.level <= Level::Error {
            println!(
                " \x1b[47m\x1b[41m ERROR \x1b[m {} | {} | {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                self.id,
                s,
            );
        }
    }
}
