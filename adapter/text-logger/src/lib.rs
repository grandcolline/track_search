use domain::port::log::Log;

#[derive(Debug, Clone)]
pub struct Logger {
    id: String,
}

impl Logger {
    pub fn new(s: String) -> Self {
        Self { id: s }
    }
}

impl Log for Logger {
    fn debug(&self, s: String) {
        println!("{} {} {}", "[DEBUG]", self.id, s);
    }

    fn info(&self, s: String) {
        println!("{} {} {}", "[INFO]", self.id, s);
    }

    fn error(&self, s: String) {
        println!("{} {} {}", "[ERROR]", self.id, s);
    }
}
