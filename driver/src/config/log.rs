use std::env;
use std::sync::Arc;

use port::LogContainer;

pub fn init() -> LogContainer {
    match env::var("LOG") {
        Ok(val) => match &*val {
            "simple" => init_simple(),
            _ => panic!("[CONFIG ERR] `{}` is invalid. founnd: {}", "LOG", val),
        },
        Err(err) => panic!("[CONFIG ERR] `{}` not get. err: {}", "LOG", err),
    }
}

fn init_simple() -> LogContainer {
    use simple::{Level, Logger};

    LogContainer {
        log: Arc::new(Logger::new("xxxxxxxx".into(), Level::Debug)),
    }
}
