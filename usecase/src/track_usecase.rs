use std::sync::Arc;

use domain::model::error::ErrorKind;
use domain::model::track_entity::TrackEntity;
use domain::port::log::Log;
use domain::port::repository::TrackRepository;

pub struct TrackUsecase<Repository: TrackRepository, Logger: Log> {
    pub repo: Arc<Repository>,
    pub log: Arc<Logger>,
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
    pub async fn get_track(&self, id: String) -> Result<TrackEntity, ErrorKind> {
        self.log
            .debug("START get track usecase. TrackID: ".to_string() + &id);

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
