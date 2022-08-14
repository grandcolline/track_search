use std::sync::Arc;

use mock::TrackGateway;
use port::RepositoryContainer;

pub fn init() -> RepositoryContainer {
    RepositoryContainer {
        track_repository: Arc::new(TrackGateway::new()),
    }
}
