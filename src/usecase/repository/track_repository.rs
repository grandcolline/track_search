use crate::entity::error::ErrorKind;
use crate::entity::track::{TrackEntity, TrackId};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait TrackRepository {
    /// IDでトラックを検索する
    async fn find_by_id(&self, id: TrackId) -> Result<TrackEntity, ErrorKind>;
}

pub trait TrackRepositoryComponent {
    type TrackRepository: TrackRepository;
    fn track_repository(&self) -> Self::TrackRepository;
}
