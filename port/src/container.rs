use std::sync::Arc;

// FIXME: crate内の参照の仕方ってこれでいいんだっけ？
use crate::log::Log;
use crate::repository::TrackRepository;

#[derive(Clone)]
pub struct Container {
    // Repository
    pub track_repository: Arc<dyn TrackRepository + Sync + Send>,

    // Log
    pub log: Arc<dyn Log + Sync + Send>,
}
