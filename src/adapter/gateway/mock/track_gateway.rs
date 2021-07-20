use crate::entity::error::ErrorKind;
use crate::entity::track::{
    Acousticness, Analysis, Artist, Danceability, Energy, Image, Instrumentalness, Liveness,
    Popularity, Speechiness, TrackEntity, TrackId, TrackTitle, Valence,
};
use crate::usecase::repository::track_repository::TrackRepository;
use async_trait::async_trait;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
pub struct TrackGateway;

#[async_trait(?Send)]
impl TrackRepository for TrackGateway {
    async fn find_by_id(&self, id: TrackId) -> Result<TrackEntity, ErrorKind> {
        let mock_entity = TrackEntity::from(
            id,
            TrackTitle::try_from("クロノスタシス".to_string())?,
            Artist::try_from("きのこ帝国".to_string())?,
            Image::try_from(
                "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141".to_string(),
            )?,
            Analysis::from(
                Popularity::try_from(58)?,
                Danceability::try_from(46)?,
                Energy::try_from(75)?,
                Valence::try_from(70)?,
                Acousticness::try_from(3)?,
                Instrumentalness::try_from(1)?,
                Liveness::try_from(13)?,
                Speechiness::try_from(3)?,
            ),
        );
        Ok(mock_entity)
    }
}
