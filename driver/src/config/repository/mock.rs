use std::sync::Arc;

use port::RepositoryContainer;

pub(crate) fn init() -> RepositoryContainer {
    RepositoryContainer {
        track_repository: Arc::new(mock::TrackGateway::new()),
    }
}
