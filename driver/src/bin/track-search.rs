//! # track-sreach
//!
//! `track-search` は楽曲の検索・情報提供を行うアプリケーションです。
//!
//! ## アーキテクチャ
//!
//! ![architecture](../../images/architecture.drawio.svg)
//!
//! * [エンティティ層](../entity/index.html)
//! * [ポート](../port/index.html)
//! * [ユースケース層](../usecase/index.html)
//! * アダプタ層
//!   * アプリケーション（プライマリアダプタ）
//!     * [html](../html/index.html)
//!     * [grpc](../grpc/index.html)
//!   * ログ
//!     * [simple](../simple/index.html)
//!     * [cloud_logging](../cloud_logging/index.html)
//!   * レポジトリ
//!     * [mock](../mock/index.html)
//!     * [spotify](../spotify/index.html)
//! * [ドライバ層](../driver/index.html)
//!
use driver::config;
use port::Container;
use std::env;

use clap::{App, Arg};
use dotenv::from_path;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // 起動CLI設定
    let app: App = App::new("track-search")
        .version("v0.1.0")
        .about("Search track application server")
        .arg(
            Arg::with_name("envfile")
                .help("envfile path")
                .long("envfile")
                .short('e')
                .takes_value(true),
        );
    let matches = app.get_matches();

    // 環境変数ファイルの読み込み
    if let Some(envfile) = matches.value_of("envfile") {
        println!("Load environment. (envfile: {})", envfile);
        from_path(envfile).ok();
    }

    // std::env::set_var("RUST_BACKTRACE", "1");

    // std::env::set_var("RUST_LOG", "actix_web=info");

    // env_logger::init();

    // let sys = actix::System::new();

    // FIXME: here!!
    let port: u16 = env::var("PORT")
        .expect("config error: PORT is required field.")
        .parse()
        .expect("config error: PORT is must be a number.");

    // Containerの作成(adpter層のDI)
    let container = Container {
        repository_container: config::repository::init(),
        log_container: config::log::init(),
    };

    if let Err(e) = match env::var("APP_ADAPTER") {
        Ok(val) => match val.as_str() {
            "html" => html::serve(port, container),
            "grpc" => grpc::serve(port, container),
            _ => panic!("[CONFIG ERROR] `{}` is invalid. founnd: {}", "APP", val),
        },
        Err(err) => panic!("[CONFIG ERROR] `{}` not get. err: {}", "APP", err),
    } {
        error!("APPLICATION START ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
