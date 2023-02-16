use entity::error::ErrorKind;
use entity::track_dto::TrackDto;
use entity::track_entity::TrackEntity;

use async_trait::async_trait;

#[async_trait]
pub trait TrackRepository {
    // crud
    async fn read(&self, id: &str) -> Result<TrackEntity, ErrorKind>;

    // find
    async fn find_by_keyword(&self, keyword: &str) -> Result<Vec<TrackDto>, ErrorKind>;
}
