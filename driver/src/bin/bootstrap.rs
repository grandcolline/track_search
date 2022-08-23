use driver::config;
use html;
use port::Container;

use clap::{App, Arg};
use dotenv::from_path;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // 起動CLI設定
    let app: App = App::new("track_search")
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

    // Containerの作成(adpter層のDI)
    let container = Container {
        repository_container: config::repository::init(),
        log_container: config::log::init(),
    };

    // VIEWアプリケーションの場合
    if let Err(e) = html::main(container) {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
