//! # ライブ度 - VO
//!
//! 楽曲のライブの数値。  
//! 0 - 100の値をとり、0が低く100が高い。
//!
//! レコーディングにオーディエンスが含まれているかで判断されている
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Liveness(u8);

impl TryFrom<u8> for Liveness {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Liveness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Liveness {
    /// ライブ音源かどうか
    /// * true:  ライブ音源
    /// * false: スタジオ音源
    pub fn is_live(&self) -> bool {
        return self.0 >= 80;
    }
}
