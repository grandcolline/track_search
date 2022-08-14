use std::sync::Arc;
use view;

use mock::TrackGateway;
use simple::Logger;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // std::env::set_var("RUST_LOG", "actix_web=info");

    // env_logger::init();

    // let sys = actix::System::new();
    //

    let modules = view::Modules {
        track_repository: Arc::new(TrackGateway::new()),
        log: Arc::new(Logger::new("xxxxxxxx".into())),
    };

    // VIEWアプリケーションの場合
    if let Err(e) = view::main(modules) {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
