use port::RepositoryContainer;
use std::env;
mod mock;

pub fn init() -> RepositoryContainer {
    match env::var("REPO_ADAPTER") {
        Ok(val) => match &*val {
            "mock" => mock::init(),
            _ => panic!("[CONFIG ERROR] `{}` is invalid. founnd: {}", "REPO", val),
        },
        Err(err) => panic!("[CONFIG ERROR] `{}` not get. err: {}", "REPO", err),
    }
}
