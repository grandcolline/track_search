mod application;
mod entity;

#[macro_use]
extern crate log;

/// 起動メソッド
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    // std::env::set_var("RUST_LOG", "actix_web=info");

    // env_logger::init();

    // let sys = actix::System::new();

    // VIEWアプリケーションの場合
    if let Err(e) = application::view::main() {
        error!("ERROR: {:?}!", e);
    }

    // let _ = sys.run();
}
