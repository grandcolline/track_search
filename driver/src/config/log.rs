use std::sync::Arc;

use port::LogContainer;
use simple::{ Logger, Level };

pub fn init() -> LogContainer {
    LogContainer {
        log: Arc::new(Logger::new("xxxxxxxx".into(), Level::Debug)),
    }
}
