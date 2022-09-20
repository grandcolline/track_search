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
    env::var("REPO_SPOTIFY_ID").expect("config error: REPO_SPOTIFY_ID is required field.")
}

fn get_secret() -> String {
    env::var("REPO_SPOTIFY_SECRET").expect("config error: REPO_SPOTIFY_SECRET is required field.")
}
