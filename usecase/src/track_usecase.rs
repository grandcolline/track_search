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
    pub async fn get_track(&self, id: &str) -> Result<TrackEntity, ErrorKind> {
        self.log
            .debug(&format!("START get track usecase. id: {}", id));

        self.repo.find_by_id(id).await
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
    pub async fn search_track(&self, key: &str) -> Result<Vec<TrackDto>, ErrorKind> {
        self.log
            .debug(&format!("START search track usecase. key: {}", key));

        self.repo.search(key).await
    }
}
