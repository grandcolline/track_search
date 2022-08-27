use std::convert::From;

use entity::key::Key;
use entity::mode::Mode;
use entity::track_entity::TrackEntity;
use port::Container;
use usecase::track_usecase::TrackUsecase;

use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct TrackResponse {
    id: String,
    title: String,
    artist: String,
    image_url: String,
    popularity: String,
    danceability: String,
    energy: String,
    valence: String,
    acousticness: String,
    instrumentalness: String,
    liveness: String,
    speechiness: String,
    time: String,
    bpm: String,
    key: String,
    mode: String,
}

impl From<TrackEntity> for TrackResponse {
    fn from(ent: TrackEntity) -> Self {
        Self {
            id: ent.id,
            title: ent.title,
            artist: ent.artist,
            image_url: ent.image,
            popularity: ent.popularity.to_string(),
            danceability: ent.danceability.to_string(),
            energy: ent.energy.to_string(),
            valence: ent.valence.to_string(),
            acousticness: ent.acousticness.to_string(),
            instrumentalness: ent.instrumentalness.to_string(),
            liveness: ent.liveness.to_string(),
            speechiness: ent.speechiness.to_string(),
            time:(ent.time / 60).to_string() + ":" + (ent.time % 60).to_string().as_str(), // FIXME
            bpm: ent.bpm.round().to_string(),
            key: match ent.key {
                Key::C => "C".to_string(),
                Key::CSahrp => "C♯".to_string(),
                Key::D => "D".to_string(),
                Key::EFlat => "E♭".to_string(),
                Key::E => "E".to_string(),
                Key::F => "F".to_string(),
                Key::FSharp => "F♯".to_string(),
                Key::G => "G".to_string(),
                Key::GSharp => "G♯".to_string(),
                Key::A => "A".to_string(),
                Key::BFlat => "B♭".to_string(),
                Key::B => "B".to_string(),
            },
            mode: match ent.mode {
                Mode::Major => "Major".to_string(),
                Mode::Minor => "Minor".to_string(),
                Mode::NoResult => "-".to_string(),
            },
        }
    }
}

#[get("/track/{id}")]
async fn track_controller(
    id: web::Path<String>,
    tera: web::Data<Tera>,
    container: web::Data<Container>,
) -> impl Responder {
    let mut context = Context::new();

    // バリデーション

    // UC作成
    let uc = TrackUsecase {
        repo: container.repository_container.track_repository.clone(),
        log: container.log_container.log.clone(),
    };

    // UC実行
    let ent = match uc.get_track(id.to_string()).await {
        Ok(t) => t,
        Err(_) => {
            // FIXME
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    // response作成
    context.insert("track", &TrackResponse::from(ent));
    let resp = match tera.render("track.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("error: {}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    // response処理
    HttpResponse::Ok().content_type("text/html").body(resp)
}
