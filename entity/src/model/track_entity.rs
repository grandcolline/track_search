use crate::model::score::Score;

// # TrackEntity
//
// 楽曲の情報や性質をもつ
//
// ### NOTE
//  * 要素を持ちすぎなので削れないか考える
//  * Entityである必要が今のところないのではないか
#[derive(Debug, Clone)]
pub struct TrackEntity {
    // TrackID
    pub id: String,
    // Track名
    pub title: String,
    // アーティスト名
    pub artist: String,
    // 画像URL
    pub image: String,
    // 人気
    pub popularity: Score,
    // 踊りやすさ
    pub danceability: Score,
    // エネルギー
    pub energy: Score,
    // ポジティブ度
    pub valence: Score,
    // アコースティック度
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

    /// ライブ音源かどうか
    /// * true:  ライブ音源
    /// * false: スタジオ音源
    pub fn is_live(&self) -> bool {
        let live = match Score::try_from(80) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.liveness >= live
    }

    /// スピーチかどうか
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
