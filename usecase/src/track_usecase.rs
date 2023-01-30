use std::sync::Arc;

use entity::error::ErrorKind;
use entity::track_dto::TrackDto;
use entity::track_entity::TrackEntity;
use port::log::Log;
use port::repository::TrackRepository;

#[derive(Clone)]
pub struct TrackUsecase {
    pub repo: Arc<dyn TrackRepository + Sync + Send>,
    pub log: Arc<dyn Log + Sync + Send>,
}

// impl<Repo: TrackRepository, Logger: Log> TrackUsecase<Repo, Logger> {
impl TrackUsecase {
    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// # Get Track
    ///
    /// get Track Infomartion by Track ID.
    ///
    /// ## Sequence
    /// ```mermaid
    /// sequenceDiagram
    ///   Application      ->> +UC get_track    : track ID
    ///   UC get_track     ->> +TrackRepository : read (track ID)
    ///   TrackRepository -->> -UC get_track    : TrackEntity
    ///   UC get_track    -->> -Application     : Result<TrackEntity, Error>
    /// ```
    pub async fn get_track(&self, id: &str) -> Result<TrackEntity, ErrorKind> {
        self.log
            .debug(&format!("START get track usecase. id: {}", id));

        self.repo.read(id).await
    }

    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// # Search Tracks
    ///
    /// Application から keyword を受け取り、その文字列を含む Track を探し、
    /// TrackDto のリストを返却する
    ///
    /// ## Sequence
    /// ```mermaid
    ///   sequenceDiagram
    ///     Application      ->> +UC search_track: keyword
    ///     UC search_track  ->> +TrackRepository: find_by_keyword (keyword)
    ///     TrackRepository -->> -UC search_track: Vec<TrackDto>
    ///     UC search_track -->> -Application    : Result<Vec<TrackDto>, Error>
    /// ```
    pub async fn search_track(&self, key: &str) -> Result<Vec<TrackDto>, ErrorKind> {
        self.log
            .debug(&format!("START search track usecase. key: {}", key));

        self.repo.find_by_keyword(key).await
    }
}
