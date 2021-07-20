//! # インストゥルメンタル度 - VO
//!
//! 楽曲のインストゥルメンタルの数値。  
//! 0 - 100の値をとり、0が低く100が高い。
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrumentalness(u8);

impl TryFrom<u8> for Instrumentalness {
    type Error = ErrorKind;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        if u <= 100 {
            Ok(Self(u))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for Instrumentalness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Instrumentalness {
    /// インスト曲かどうか
    /// * true:  インスト曲
    /// * false: ボーカル曲
    pub fn is_instrument(&self) -> bool {
        // インスト度が50以上の場合、インスト曲とみなす
        return self.0 >= 50;
    }
}
