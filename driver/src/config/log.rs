use port::LogContainer;
use std::env;

mod cloud_logging;
mod simple;

pub fn init() -> LogContainer {
    match env::var("LOG_ADAPTER") {
        Ok(val) => match &*val {
            "simple" => simple::init(),
            "cloud_logging" => cloud_logging::init(),
            _ => panic!("[CONFIG ERROR] `{}` is invalid. founnd: {}", "LOG", val),
        },
        Err(err) => panic!("[CONFIG ERROR] `{}` not get. err: {}", "LOG", err),
    }
}
