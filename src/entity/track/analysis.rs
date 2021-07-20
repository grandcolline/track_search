//! 分析データ - VO
use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Analysis {
    /// 人気
    popularity: Popularity,
    /// 踊りやすさ
    danceability: Danceability,
    /// エネルギー
    energy: Energy,
    /// ボジティブ
    valence: Valence,
    /// アコースティック
    acousticness: Acousticness,
    /// インストゥルメンタル
    instrumentalness: Instrumentalness,
    /// ライブ感
    liveness: Liveness,
    /// ポエトリー
    speechiness: Speechiness,
}

impl Analysis {
    /// 分析データセットVOを生成する
    pub fn from(
        popularity: Popularity,
        danceability: Danceability,
        energy: Energy,
        valence: Valence,
        acousticness: Acousticness,
        instrumentalness: Instrumentalness,
        liveness: Liveness,
        speechiness: Speechiness,
    ) -> Self {
        Self {
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
    pub fn popularity(&self) -> &Popularity {
        &self.popularity
    }

    pub fn danceability(&self) -> &Danceability {
        &self.danceability
    }

    pub fn energy(&self) -> &Energy {
        &self.energy
    }

    pub fn valence(&self) -> &Valence {
        &self.valence
    }

    pub fn acousticness(&self) -> &Acousticness {
        &self.acousticness
    }

    pub fn instrumentalness(&self) -> &Instrumentalness {
        &self.instrumentalness
    }

    pub fn liveness(&self) -> &Liveness {
        &self.liveness
    }

    pub fn speechiness(&self) -> &Speechiness {
        &self.speechiness
    }
}
