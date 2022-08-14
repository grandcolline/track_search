use entity::model::error::ErrorKind;
use entity::model::track_entity::TrackEntity;

use async_trait::async_trait;

#[async_trait]
pub trait TrackRepository {
    /// IDでトラックを検索する
    async fn find_by_id(&self, id: String) -> Result<TrackEntity, ErrorKind>;
}
