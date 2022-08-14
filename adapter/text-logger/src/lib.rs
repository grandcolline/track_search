use port::log::Log;

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
        println!("\x1b[47m\x1b[45m DEBUG \x1b[m {} | {}", self.id, s);
    }

    fn info(&self, s: String) {
        println!("\x1b[47m\x1b[44m INFO  \x1b[m {} | {}", self.id, s);
    }

    // fn warn(&self, s: String) {
    //     println!("\x1b[47m\x1b[43m WARN  \x1b[m {} | {}", self.id, s);
    // }

    fn error(&self, s: String) {
        println!("\x1b[47m\x1b[41m ERROR \x1b[m {} | {}", self.id, s);
    }
}
