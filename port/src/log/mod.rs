pub trait Log {
    /// DEBUGログを出力する
    fn debug(&self, s: &str);
    /// INFOログを出力する
    fn info(&self, s: &str);
    /// ERRORログを出力する
    fn error(&self, s: &str);
}
