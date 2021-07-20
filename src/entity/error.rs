//! # エラー
//!
//! エラーの定義を行なっている
//!
//! ## 参考
//! * [[Rust]自作Errorを作る - Qiita](https://qiita.com/mutuya/items/b09881839c4e02b2f485)
use std::error;
use std::fmt;

/// エラー区分
#[derive(Clone)]
pub enum ErrorKind {
    TypeError,
    NotFound,
}

impl ErrorKind {
    pub fn description(&self) -> &'static str {
        match self {
            ErrorKind::TypeError => "型が複製できませんでした",
            ErrorKind::NotFound => "存在しません",
        }
    }
}

pub struct Error {
    _error: _Error,
}

impl Error {
    fn new<E>(kind: ErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Error {
            _error: _Error::Custom((kind, error.into())),
        }
    }
}

enum _Error {
    Simple(ErrorKind),
    Custom((ErrorKind, Box<dyn error::Error + Send + Sync>)),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._error {
            _Error::Simple(s) => f.write_str(s.description()),
            _Error::Custom(c) => f.write_str(c.0.description()),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self._error {
            _Error::Simple(_) => None,
            _Error::Custom(c) => c.1.source(),
        }
    }
}
