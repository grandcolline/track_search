//! # Port Layer
//!
//! ユースケース層で使われる副作用を目的とした処理のインターフェース。  
//! 実実装はアダプター層にもつ。

pub mod log;
pub mod repository;

pub mod container;
pub use container::{Container, LogContainer, RepositoryContainer};
