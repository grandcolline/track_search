use crate::usecase::log::log::Log;

#[derive(Debug, Copy, Clone)]
pub struct Logger;

impl Log for Logger {
    fn debug(&self, s: String) {
        println!("{} {}", "[DEBUG]", s);
    }

    fn info(&self, s: String) {
        println!("{} {}", "[INFO]", s);
    }

    fn error(&self, s: String) {
        println!("{} {}", "[ERROR]", s);
    }
}
