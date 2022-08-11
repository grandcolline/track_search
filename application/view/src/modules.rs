use std::sync::Arc;

use domain::port::log::Log;
use domain::port::repository::TrackRepository;

#[derive(Clone)]
pub struct Modules {
   pub track_repository: Arc<dyn TrackRepository + Sync + Send>,
   pub log: Arc<dyn Log + Sync + Send>,
   // pub track_repository: TrackRepository,
   // pub log: Log,
}

