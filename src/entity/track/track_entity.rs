//! トラックエンティティ - Entity
use super::*;

#[derive(Debug, Clone)]
pub struct TrackEntity {
    pub id: TrackId,
    pub title: TrackTitle,
    pub artist: Artist,
    pub image: Image,
    pub analysis: Analysis,
}
impl TrackEntity {
    pub fn from(
        id: TrackId,
        title: TrackTitle,
        artist: Artist,
        image: Image,
        analysis: Analysis,
    ) -> Self {
        Self {
            id,
            title,
            artist,
            image,
            analysis,
        }
    }
}
