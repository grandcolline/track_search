// FIXME: どこの層におくべきかよく考える!!!!!!!!!
#[derive(Debug, Clone)]
pub struct TrackDto {
    /// Track ID
    pub id: String,
    /// Track Name
    pub title: String,
    /// Artist Name
    pub artist: String,
    /// Track Image URL
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
