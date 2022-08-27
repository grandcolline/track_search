use entity::error::ErrorKind;
use entity::key::Key;
use entity::mode::Mode;
use entity::score::Score;
use entity::track_entity::TrackEntity;
use port::repository::TrackRepository;

use async_trait::async_trait;
use rspotify::{model::Modality, model::TrackId, prelude::*, ClientCredsSpotify, Credentials};

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
    async fn find_by_id(&self, id: String) -> Result<TrackEntity, ErrorKind> {
        let mut spotify = ClientCredsSpotify::new(self.creds.clone());
        spotify.request_token().await.unwrap();

        // Running the requests
        // id: 299XX9C7B56JuqGtokDeap
        let track_id = TrackId::from_id(&id).unwrap();
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
            id,
            track.name,
            track.artists[0].name.clone(),
            track.album.images[0].url.clone(),
            translate_mode(feature.mode),
            translate_key(feature.key)?,
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
}

fn translate_mode(mode: Modality) -> Mode {
    match mode {
        Modality::Major => Mode::Major,
        Modality::Minor => Mode::Minor,
        Modality::NoResult => Mode::NoResult,
    }
}

fn translate_key(key: i32) -> Result<Key, ErrorKind> {
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
