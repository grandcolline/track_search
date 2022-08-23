use std::env;
use std::sync::Arc;

use port::RepositoryContainer;

pub fn init() -> RepositoryContainer {
    match env::var("REPO_ADAPTER") {
        Ok(val) => match &*val {
            "mock" => init_mock(),
            _ => panic!("[CONFIG ERR] `{}` is invalid. founnd: {}", "REPO", val),
        },
        Err(err) => panic!("[CONFIG ERR] `{}` not get. err: {}", "REPO", err),
    }
}

fn init_mock() -> RepositoryContainer {
    RepositoryContainer {
        track_repository: Arc::new(mock::TrackGateway::new()),
    }
}
