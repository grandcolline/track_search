//! # Entity Layer
//!
//! 最重要ビジネスルールを記述する層。  
//! 例外はあるが、この層は基本的に下記の２つの要素を持つ。
//! * Value Object
//!   * 等価性によって比較される(アイデンティティを持たない)
//!   * 不変(イミュータブル)である
//! * Entity
//!   * アイデンティティにより区別される
//!   * 可変である
//!
//! この層は内部・外部問わず、他のクレートに依存しないような構成にしたい。
pub mod error;
pub mod key;
pub mod mode;
pub mod score;
pub mod track_dto;
pub mod track_entity;
