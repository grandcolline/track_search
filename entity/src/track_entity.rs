use crate::key::Key;
use crate::mode::Mode;
use crate::score::Score;

/// # TrackEntity
///
/// 楽曲の情報や性質をもつ
///
/// ### NOTE
///  * 要素を持ちすぎなので削れないか考える
///  * Entityである必要が今のところないのではないか
#[derive(Debug, Clone)]
pub struct TrackEntity {
    /// TrackID
    pub id: String,
    /// Track名
    pub title: String,
    /// アーティスト名
    pub artist: String,
    /// Track画像URL
    pub image: String,
    /// モード(旋法)
    pub mode: Mode,
    /// キー
    pub key: Key,
    /// BPM
    pub bpm: f32,
    /// 再生時間(秒)
    pub time: u64,
    /// 人気度
    pub popularity: Score,
    /// 踊りやすさ
    pub danceability: Score,
    /// エネルギー
    pub energy: Score,
    /// ポジティブ度
    pub valence: Score,
    /// アコースティック度
    pub acousticness: Score,
    /// インストゥルメンタル
    pub instrumentalness: Score,
    /// ライブ感
    pub liveness: Score,
    /// ポエトリー
    pub speechiness: Score,
}

impl TrackEntity {
    pub fn from(
        id: String,
        title: String,
        artist: String,
        image: String,
        mode: Mode,
        key: Key,
        bpm: f32,
        time: u64,
        popularity: Score,
        danceability: Score,
        energy: Score,
        valence: Score,
        acousticness: Score,
        instrumentalness: Score,
        liveness: Score,
        speechiness: Score,
    ) -> Self {
        Self {
            id,
            title,
            artist,
            image,
            mode,
            key,
            bpm,
            time,
            popularity,
            danceability,
            energy,
            valence,
            acousticness,
            instrumentalness,
            liveness,
            speechiness,
        }
    }

    /// このTrackがライブ音源かどうかを返す
    /// * true:  ライブ音源
    /// * false: スタジオ音源
    pub fn is_live(&self) -> bool {
        let live = match Score::try_from(80) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.liveness >= live
    }

    /// このTrackがスピーチかどうかを返す
    /// * true:  スピーチ
    /// * false: スピーチではない
    pub fn is_speech(&self) -> bool {
        let live = match Score::try_from(33) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.speechiness >= live
    }
}
