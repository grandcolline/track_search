pub trait Log {
    /// print log message of DEBUG level
    fn debug(&self, s: &str);
    /// print log message of INFO  level
    fn info(&self, s: &str);
    /// print log message of ERROR level
    fn error(&self, s: &str);
}
