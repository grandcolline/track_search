//! # スピーチ度 - VO
//!
//! 楽曲のスピーチの数値。  
//! 0 - 100の値をとり、0が低く100が高い。
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speechiness(u8);

impl TryFrom<u8> for Speechiness {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Speechiness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Speechiness {
    /// スピーチかどうか
    /// * true:  スピーチ
    /// * false: スピーチではない
    pub fn is_speech(&self) -> bool {
        return self.0 >= 33;
    }
}
