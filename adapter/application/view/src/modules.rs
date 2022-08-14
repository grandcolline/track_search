use std::sync::Arc;

use port::log::Log;
use port::repository::TrackRepository;

#[derive(Clone)]
pub struct Modules {
    // Repository
    pub track_repository: Arc<dyn TrackRepository + Sync + Send>,

    // Log
    pub log: Arc<dyn Log + Sync + Send>,
}
