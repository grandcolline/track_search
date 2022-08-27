use port::RepositoryContainer;
use std::env;

mod mock;
mod spotify;

pub fn init() -> RepositoryContainer {
    match env::var("REPO_ADAPTER") {
        Ok(val) => match val.as_str() {
            "mock" => mock::init(),
            "spotify" => spotify::init(),
            _ => panic!("[CONFIG ERROR] `{}` is invalid. founnd: {}", "REPO", val),
        },
        Err(err) => panic!("[CONFIG ERROR] `{}` not get. err: {}", "REPO", err),
    }
}
