use entity::error::ErrorKind;
use entity::key::Key;
use entity::mode::Mode;
use entity::score::Score;
use entity::track_dto::TrackDto;
use entity::track_entity::TrackEntity;
use port::repository::TrackRepository;

use async_trait::async_trait;
use rspotify::{
    model::Modality, model::SearchResult, model::SearchType, model::TrackId, prelude::*,
    ClientCredsSpotify, Credentials,
};

#[derive(Debug, Clone)]
pub struct TrackGateway {
    creds: Credentials,
}

impl TrackGateway {
    pub fn new(id: &str, secret: &str) -> Self {
        Self {
            creds: Credentials::new(id, secret),
        }
    }
}

#[async_trait]
impl TrackRepository for TrackGateway {
    async fn find_by_id(&self, id: &str) -> Result<TrackEntity, ErrorKind> {
        let mut spotify = ClientCredsSpotify::new(self.creds.clone());
        spotify.request_token().await.unwrap();

        // Running the requests
        let track_id = TrackId::from_id(id).unwrap();
        let track = match spotify.track(&track_id).await {
            Ok(track) => track,
            Err(_) => return Err(ErrorKind::NotFound),
        };

        let feature = match spotify.track_features(&track_id).await {
            Ok(v) => v,
            Err(_) => return Err(ErrorKind::NotFound),
        };

        // println!("{:?}", track);
        // println!("{:?}", feature);

        Ok(TrackEntity::from(
            id.into(),
            track.name,
            track.artists[0].name.clone(),
            track.album.images[0].url.clone(),
            cnv_mode(feature.mode),
            cnv_key(feature.key)?,
            feature.tempo,
            track.duration.as_secs(),
            Score::try_from(track.popularity as u8)?,
            Score::try_from((feature.danceability * 100.0).round() as u8)?,
            Score::try_from((feature.energy * 100.0).round() as u8)?,
            Score::try_from((feature.valence * 100.0).round() as u8)?,
            Score::try_from((feature.acousticness * 100.0).round() as u8)?,
            Score::try_from((feature.instrumentalness * 100.0).round() as u8)?,
            Score::try_from((feature.liveness * 100.0).round() as u8)?,
            Score::try_from((feature.speechiness * 100.0).round() as u8)?,
        ))
    }

    async fn search(&self, key: &str) -> Result<Vec<TrackDto>, ErrorKind> {
        if key.is_empty() {
            return Err(ErrorKind::NotFound);
        }

        let mut spotify = ClientCredsSpotify::new(self.creds.clone());
        spotify.request_token().await.unwrap();

        let kekka = match spotify
            .search(key, &SearchType::Track, None, None, Some(15), None)
            .await
        {
            Ok(v) => match v {
                SearchResult::Tracks(x) => x.items,
                _ => return Err(ErrorKind::NotFound),
            },
            Err(_) => return Err(ErrorKind::NotFound),
        };

        // println!("{:?}", kekka);

        let mut res = vec![];
        for record in kekka.iter() {
            res.push(TrackDto::from(
                match &record.id {
                    Some(id) => id.id().to_string(),
                    None => continue,
                },
                record.name.clone(),
                record.artists[0].name.clone(),
                record.album.images[0].url.clone(),
            ));
        }
        Ok(res)
    }
}

fn cnv_mode(mode: Modality) -> Mode {
    match mode {
        Modality::Major => Mode::Major,
        Modality::Minor => Mode::Minor,
        Modality::NoResult => Mode::NoResult,
    }
}

fn cnv_key(key: i32) -> Result<Key, ErrorKind> {
    match key {
        0 => Ok(Key::C),
        1 => Ok(Key::CSahrp),
        2 => Ok(Key::D),
        3 => Ok(Key::EFlat),
        4 => Ok(Key::E),
        5 => Ok(Key::F),
        6 => Ok(Key::FSharp),
        7 => Ok(Key::G),
        8 => Ok(Key::GSharp),
        9 => Ok(Key::A),
        10 => Ok(Key::BFlat),
        11 => Ok(Key::B),
        _ => Err(ErrorKind::TypeError),
    }
}
