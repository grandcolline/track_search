use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

// # スコア - VO
//
// 曲の性質の数値  
// 0 - 100の値をとり、0が低く100が高い。
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(u8);

impl TryFrom<u8> for Score {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
