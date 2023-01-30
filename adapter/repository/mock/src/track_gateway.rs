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

impl Default for TrackGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TrackRepository for TrackGateway {
    async fn read(&self, id: &str) -> Result<TrackEntity, ErrorKind> {
        Ok(TrackEntity::from(
            id.into(),
            "クロノスタシス".into(),
            "きのこ帝国".into(),
            "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".into(),
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

    async fn find_by_keyword(&self, _: &str) -> Result<Vec<TrackDto>, ErrorKind> {
        Ok(vec![
            TrackDto::from(
                "aaaaa".into(),
                "クロノスタシス".into(),
                "きのこ帝国".into(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".into(),
            ),
            TrackDto::from(
                "bbbbbb".into(),
                "クロノスタシス2".into(),
                "きのこ帝国".into(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".into(),
            ),
            TrackDto::from(
                "ccccccc".into(),
                "クロノスタシス3".into(),
                "きのこ帝国3".into(),
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".into(),
            ),
        ])
    }
}
