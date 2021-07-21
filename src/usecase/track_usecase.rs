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
    /// # 楽曲を取得する
    ///
    /// ## Prams
    /// * トラックID
    ///
    /// ## Return
    /// * Result
    ///   * トラックエンティティ
    ///   * ErrorKind
    ///
    /// ## Sequence
    /// ```mermaid
    /// sequenceDiagram
    ///   アプリケーション     ->> +UC「楽曲を取得する」 : トラックID
    ///   UC「楽曲を取得する」 ->> +トラックレポジトリ   : IDでトラックを検索する(トラックID)
    ///   トラックレポジトリ   ->> -UC「楽曲を取得する」 : トラックエンティティ
    ///   UC「楽曲を取得する」 ->> -アプリケーション     : Result<トラックエンティティ, エラー>
    /// ```
    pub async fn get_track(&self, id: TrackId) -> Result<TrackEntity, ErrorKind> {
        self.log
            .debug("START get track usecase. TrackID: ".to_string() + &id.to_string());

        return self.repo.find_by_id(id).await;
    }

    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// # 楽曲を検索する
    ///
    /// ## Prams
    /// * キーワード - String
    ///
    /// ## Return
    /// * Result
    ///   * トラックエンティティリスト
    ///   * ErrorKind
    ///
    /// ## Sequence
    /// ```mermaid
    /// sequenceDiagram
    ///   アプリケーション     ->> +UC「楽曲を検索する」 : キーワード
    ///   UC「楽曲を検索する」 ->> +トラックレポジトリ   : トラックを検索する(キーワード)
    ///   トラックレポジトリ   ->> -UC「楽曲を検索する」 : トラックエンティティリスト
    ///   UC「楽曲を検索する」 ->> -アプリケーション     : Result<トラックエンティティリスト, エラー>
    /// ```
    pub async fn search_track(&self) {
        // TODO
    }
}
