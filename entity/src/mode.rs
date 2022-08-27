use super::error::ErrorKind;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Major,
    Minor,
    NoResult,
}

impl TryFrom<String> for Mode {
    type Error = ErrorKind;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "Major" | "MAJOR" | "major" => Ok(Mode::Major),
            "Minor" | "MINOR" | "minor" => Ok(Mode::Minor),
            _ => Ok(Mode::NoResult),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
