//! # トラック集約
//!
//! トラック(楽曲)に関するビジネスロジックを定義
//!
//! [モデル図](./fn._diagram.html)
//!

#[cfg_attr(doc, aquamarine::aquamarine)]
/// # トラック集約 ドメインモデル図
///
/// ```mermaid
/// classDiagram
///   class 分析データ{
///     -アコースティック度
///     -踊りやすさ度
///     -エネルギー度
///     -インストゥルメンタル度
///     -ライブ度
///     -人気度
///     -スピーチ度
///     -ポジティブ度
///     +インスト曲かどうか() Bool
///     +ライブ音源かどうか() Bool
///     +スピーチかどうか() Bool
///   }
///   class トラックエンティティ{
///     -トラックID
///     -トラック名
///     -画像
///     -アーティスト
///     -分析データ
///   }
///   トラックエンティティ o-- 分析データ
/// ```
fn _diagram() {}

// Value Object
mod score;

// Entity
mod track_entity;

// use
pub use track_entity::*;
pub use score::*;

