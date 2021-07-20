pub trait Log {
    /// DEBUGログを出力する
    fn debug(&self, s: String);
    /// INFOログを出力する
    fn info(&self, s: String);
    /// ERRORログを出力する
    fn error(&self, s: String);
}
