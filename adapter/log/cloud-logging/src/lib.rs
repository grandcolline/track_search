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
    trace: String,
    tracking_id: String,
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
            match serde_json::to_string(&LogTemplate {
                severity: "DEBUG".to_owned(),
                message: s,
                time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                trace: "".to_owned(), // TODO
                tracking_id: self.id.clone(),
            }) {
                Ok(json) => println!("{}", json),
                Err(_) => panic!(),
            }
        }
    }

    fn info(&self, s: String) {
        if self.level <= Level::Info {
            match serde_json::to_string(&LogTemplate {
                severity: "INFO".to_owned(),
                message: s,
                time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                trace: "".to_owned(), // TODO
                tracking_id: self.id.clone(),
            }) {
                Ok(json) => println!("{}", json),
                Err(_) => panic!(),
            }
        }
    }

    fn error(&self, s: String) {
        if self.level <= Level::Error {
            match serde_json::to_string(&LogTemplate {
                severity: "ERROR".to_owned(),
                message: s,
                time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                trace: "".to_owned(), // TODO
                tracking_id: self.id.clone(),
            }) {
                Ok(json) => println!("{}", json),
                Err(_) => panic!(),
            }
        }
    }
}
