//! # Track Search
//!
//! Rustで作成された、楽曲を検索して情報を提供するWEBアプリケーション
//!
//! 起動方法や運用などについては
//! [README](https://github.com/grandcolline/track_search/blob/master/README.md)に、
//! アーキテクチャや実装方法などについては
//! [RustDoc](./)
//! にドキュメントとしてまとめる。
//!
//! ## 🤖 アーキテクチャ
//!
//! 5つの層(Layer)から構成される。
//!
//! | Name                        | Note                                                |
//! |:----------------------------|:----------------------------------------------------|
//! | [エンティティ層](./entity)  | ビジネスロジックを定義する層                        |
//! | [ユースケース層](./usecase) | ビジネスフロー/アプリケーションロジックを定義する層 |
//! | [アプリケーション層](./application) | ユーザとユースケースをつなぐ層              |
//! | [アダプター層](./adapter)   | ユースケース層で定義されたポートの実装を持つ層      |
//! | [ドライバー層](./driver)    | アプリケーションとアダプターの構築を行う層          |
//!
//! ### 依存関係
//!
//! ```
//! ENTITY <- USECASE <- APPLICATION / ADAPTER <- DRIVER
//! ```
//! ※ アプリケーション層とアダプター層はそれぞれドライバー層から参照され、
//! ユースケース層に依存するが、互いには依存し合わない。  
//! ※ 各レイヤーはsrc直下にモジュールとして配置する。  
//! ※ レイヤー内の依存関係は、親モジュールが子モジュールに依存するような関係となる  
//!

mod adapter;
mod application;
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
    if let Err(e) = application::view::build() {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
