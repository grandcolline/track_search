use std::sync::Arc;

use crate::log::Log;
use crate::repository::TrackRepository;

#[derive(Clone)]
pub struct RepositoryContainer {
    pub track_repository: Arc<dyn TrackRepository + Sync + Send>,
}

#[derive(Clone)]
pub struct LogContainer {
    pub log: Arc<dyn Log + Sync + Send>,
}

#[derive(Clone)]
pub struct Container {
    pub repository_container: RepositoryContainer,
    pub log_container: LogContainer,
}
