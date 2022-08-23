use std::sync::Arc;

use cloud_logging::{Level, Logger};
use port::LogContainer;

pub(crate) fn init() -> LogContainer {
    LogContainer {
        log: Arc::new(Logger::new("xxxxxxxx".into(), Level::Debug)),
    }
}
