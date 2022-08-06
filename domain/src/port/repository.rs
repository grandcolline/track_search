use crate::model::error::ErrorKind;
use crate::model::track_entity::TrackEntity;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait TrackRepository {
    /// IDでトラックを検索する
    async fn find_by_id(&self, id: String) -> Result<TrackEntity, ErrorKind>;
}

pub trait TrackRepositoryComponent {
    type TrackRepository: TrackRepository;
    fn track_repository(&self) -> Self::TrackRepository;
}
