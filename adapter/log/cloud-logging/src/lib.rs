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
    fn debug(&self, s: &str) {
        if self.level <= Level::Debug {
            println!("{}", create_json("DEBUG".to_owned(), s, &self.id));
        }
    }

    fn info(&self, s: &str) {
        if self.level <= Level::Info {
            println!("{}", create_json("INFO".to_owned(), s, &self.id));
        }
    }

    fn error(&self, s: &str) {
        if self.level <= Level::Error {
            println!("{}", create_json("ERROR".to_owned(), s, &self.id));
        }
    }
}

fn create_json(severity: String, message: &str, tracking_id: &str) -> String {
    match serde_json::to_string(&LogTemplate {
        severity,
        message: message.to_string(),
        time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        tracking_id: tracking_id.to_string(),
        // trace: "".to_owned(),
    }) {
        Ok(json) => json,
        Err(_) => "log formatting error!".to_string(), // FIXME: error内容も表示する
    }
}
