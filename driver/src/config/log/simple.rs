use std::sync::Arc;

use port::LogContainer;
use simple::{Level, Logger};

pub(crate) fn init() -> LogContainer {
    LogContainer {
        log: Arc::new(Logger::new("xxxxxxxx".into(), Level::Debug)),
    }
}
