use std::env;
use std::sync::Arc;

use port::RepositoryContainer;

pub(crate) fn init() -> RepositoryContainer {
    RepositoryContainer {
        track_repository: Arc::new(spotify::TrackGateway::new(&get_id(), &get_secret())),
    }
}

// const PREFIX: &str = "REPO_SPOTIFY_";

fn get_id() -> String {
    match env::var("REPO_SPOTIFY_ID") {
        Ok(val) => val,
        Err(_) => panic!("FIXME!! REPO_SPOTIFY_ID"), // FIXME
    }
}

fn get_secret() -> String {
    match env::var("REPO_SPOTIFY_SECRET") {
        Ok(val) => val,
        Err(_) => panic!("FIXME!! REPO_SPOTIFY_SECRET"), // FIXME
    }
}
