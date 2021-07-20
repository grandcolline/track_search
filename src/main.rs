//! # Track Search
//!
//! Rustで作成された、楽曲を検索して情報を提供するWEBアプリケーション
//!
//! 起動方法や運用などについては
//! [README](https://github.com/grandcolline/track_search/blob/master/README.md)に、
//! アーキテクチャや実装方法などについてはRustDocにドキュメントとしてまとめる。
//!
//! ## アーキテクチャ
//! 4つの層(Layer)から構成され、依存関係は次の図のようになっている。
//!
//! TODO

mod adapter;
mod entity;
mod error;
mod usecase;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // std::env::set_var("RUST_LOG", "actix_web=info");

    // env_logger::init();

    // let sys = actix::System::new();

    // RESTアプリケーションの場合
    // if let Err(e) = application::rest::build() {
    //     error!("ERROR: {:?}!", e);
    // }

    // VIEWアプリケーションの場合
    if let Err(e) = adapter::controller::view::build() {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
