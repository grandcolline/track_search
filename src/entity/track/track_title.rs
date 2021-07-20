//! # トラック名 - VO
use crate::entity::error::ErrorKind;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrackTitle(String);

impl TryFrom<String> for TrackTitle {
    type Error = ErrorKind;

    /// TrackId 復元メソッド
    fn try_from(s: String) -> Result<Self, Self::Error> {
        // 空文字でない
        if 0 < s.chars().count() {
            Ok(Self(s))
        } else {
            Err(ErrorKind::TypeError)
        }
    }
}

impl fmt::Display for TrackTitle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
