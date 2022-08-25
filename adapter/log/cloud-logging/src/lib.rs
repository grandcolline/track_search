use chrono::Local;
use port::log::Log;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum Level {
    Debug,
    Info,
    Error,
}

#[derive(Serialize, Debug)]
struct LogTemplate {
    severity: String,
    message: String,
    time: String,
    tracking_id: String,
    // trace: String,
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
    fn debug(&self, s: String) {
        if self.level <= Level::Debug {
            println!("{}", create_json("DEBUG".to_owned(), s, self.id.clone()));
        }
    }

    fn info(&self, s: String) {
        if self.level <= Level::Info {
            println!("{}", create_json("INFO".to_owned(), s, self.id.clone()));
        }
    }

    fn error(&self, s: String) {
        if self.level <= Level::Error {
            println!("{}", create_json("ERROR".to_owned(), s, self.id.clone()));
        }
    }
}

fn create_json(severity: String, message: String, tracking_id: String) -> String {
    match serde_json::to_string(&LogTemplate {
        severity,
        message,
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        tracking_id,
        // trace: "".to_owned(),
    }) {
        Ok(json) => json,
        Err(_) => "log formatting error!".to_string(), // FIXME: error内容も表示する
    }
}
