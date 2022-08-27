use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    C,
    CSahrp,
    D,
    EFlat,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    BFlat,
    B,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
