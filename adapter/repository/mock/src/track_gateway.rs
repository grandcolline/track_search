use entity::error::ErrorKind;
use entity::key::Key;
use entity::mode::Mode;
use entity::score::Score;
use entity::track_dto::TrackDto;
use entity::track_entity::TrackEntity;
use port::repository::TrackRepository;

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
            Mode::Minor,
            Key::CSahrp,
            83.9,
            252,
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

    async fn search(&self, key: String) -> Result<Vec<TrackDto>, ErrorKind> {
        Ok(vec![
            TrackDto::from(
                "aaaaa".to_string(),
                "クロノスタシス".to_string(),
                "きのこ帝国".to_string(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".to_string(),
            ),
            TrackDto::from(
                "bbbbbb".to_string(),
                "クロノスタシス2".to_string(),
                "きのこ帝国".to_string(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".to_string(),
            ),
            TrackDto::from(
                "ccccccc".to_string(),
                "クロノスタシス3".to_string(),
                "きのこ帝国3".to_string(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".to_string(),
            ),
        ])
    }
}
