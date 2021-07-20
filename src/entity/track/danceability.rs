//! # 踊りやすさ度 - VO
//!
//! 楽曲の踊りやすさの数値。  
//! 0 - 100の値をとり、0が低く100が高い。
//!
//! テンポ、リズムの一定感、ビートの強さなどから算出
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Danceability(u8);

impl TryFrom<u8> for Danceability {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Danceability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
