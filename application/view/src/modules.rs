use std::sync::Arc;

use port::log::Log;
use port::repository::TrackRepository;

#[derive(Clone)]
pub struct Modules {
    pub track_repository: Arc<dyn TrackRepository + Sync + Send>,
    pub log: Arc<dyn Log + Sync + Send>,
}
