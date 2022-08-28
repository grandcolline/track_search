// FIXME: どこの層におくべきかよく考える!!!!!!!!!
#[derive(Debug, Clone)]
pub struct TrackDto {
    /// TrackID
    pub id: String,
    /// Track名
    pub title: String,
    /// アーティスト名
    pub artist: String,
    /// Track画像URL
    pub image: String,
}

impl TrackDto {
    pub fn from(id: String, title: String, artist: String, image: String) -> Self {
        Self {
            id,
            title,
            artist,
            image,
        }
    }
}
