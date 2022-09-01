use entity::error::ErrorKind;
use entity::track_dto::TrackDto;
use entity::track_entity::TrackEntity;

use async_trait::async_trait;

#[async_trait]
pub trait TrackRepository {
    /// IDでトラックを検索する
    async fn find_by_id(&self, id: &str) -> Result<TrackEntity, ErrorKind>;
    async fn search(&self, key: &str) -> Result<Vec<TrackDto>, ErrorKind>;
}
