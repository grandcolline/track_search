use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    NoResult,
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Major" | "MAJOR" | "major" => Mode::Major,
            "Minor" | "MINOR" | "minor" => Mode::Minor,
            _ => Mode::NoResult,
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
