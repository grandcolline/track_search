use driver::config;
use port::Container;
use view;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // std::env::set_var("RUST_LOG", "actix_web=info");

    // env_logger::init();

    // let sys = actix::System::new();

    let container = Container {
        repository_container: config::repository::init(),
        log_container: config::log::init(),
    };

    // VIEWアプリケーションの場合
    if let Err(e) = view::main(container) {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
