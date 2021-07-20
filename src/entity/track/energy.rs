//! # エネルギー度 - VO
//!
//! 楽曲のエネルギーの数値。  
//! 0 - 100の値をとり、0が低く100が高い。
//!
//! fast, loud, noisyであれば100に近づく
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Energy(u8);

impl TryFrom<u8> for Energy {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
