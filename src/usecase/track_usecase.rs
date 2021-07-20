use crate::entity::error::ErrorKind;
use crate::entity::track::{TrackEntity, TrackId};
use crate::usecase::log::log::Log;
use crate::usecase::repository::track_repository::TrackRepository;

pub struct TrackUsecase<Repository: TrackRepository, Logger: Log> {
    pub repo: Repository,
    pub log: Logger,
}

impl<Repo: TrackRepository, Logger: Log> TrackUsecase<Repo, Logger> {
    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// 楽曲を取得する
    pub async fn get_track(&self, id: TrackId) -> Result<TrackEntity, ErrorKind> {
        self.log
            .debug("START get track usecase. TrackID: ".to_string() + &id.to_string());

        return self.repo.find_by_id(id).await;
    }

    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// 楽曲を検索する
    pub async fn search_track(&self, id: TrackId) -> Result<TrackEntity, ErrorKind> {
        self.log.info("START search track usecase".to_string());
        return self.repo.find_by_id(id).await;
    }
}
