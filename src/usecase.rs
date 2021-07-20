//! # ユースケース層 (Usecase Layer)
//!
//! Application Business Rulesの責務を担当する層。  
//!
//! この層ではなるべく外部パッケージは使わない。
//! 使うクレートは[Standard Library(std)](https://doc.rust-lang.org/std/)ぐらいにする。
//!
// port
pub mod log;
pub mod repository;

// usecase
pub mod track_usecase;
