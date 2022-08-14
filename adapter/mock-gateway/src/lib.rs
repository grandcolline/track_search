use domain::model::error::ErrorKind;
use domain::model::score::Score;
use domain::model::track_entity::TrackEntity;
use domain::port::repository::TrackRepository;

use async_trait::async_trait;

#[derive(Debug, Copy, Clone)]
pub struct TrackGateway;

impl TrackGateway {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl TrackRepository for TrackGateway {
    async fn find_by_id(&self, id: String) -> Result<TrackEntity, ErrorKind> {
        Ok(TrackEntity::from(
            id,
            "クロノスタシス".to_string(),
            "きのこ帝国".to_string(),
            "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".to_string(),
            Score::try_from(58)?,
            Score::try_from(46)?,
            Score::try_from(75)?,
            Score::try_from(70)?,
            Score::try_from(3)?,
            Score::try_from(1)?,
            Score::try_from(13)?,
            Score::try_from(3)?,
        ))
    }
}
