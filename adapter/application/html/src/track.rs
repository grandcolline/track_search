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
            time: format!("{:?}:{:02}", ent.time/60, ent.time%60),
            bpm: ent.bpm.round().to_string(),
            key: match ent.key {
                Key::C => "C".into(),
                Key::CSahrp => "C♯".into(),
                Key::D => "D".into(),
                Key::EFlat => "E♭".into(),
                Key::E => "E".into(),
                Key::F => "F".into(),
                Key::FSharp => "F♯".into(),
                Key::G => "G".into(),
                Key::GSharp => "G♯".into(),
                Key::A => "A".into(),
                Key::BFlat => "B♭".into(),
                Key::B => "B".into(),
            },
            mode: match ent.mode {
                Mode::Major => "Major".into(),
                Mode::Minor => "Minor".into(),
                Mode::NoResult => "-".into(),
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

    // 入力値チェック

    // UC作成
    let uc = TrackUsecase {
        repo: container.repository_container.track_repository.clone(),
        log: container.log_container.log.clone(),
    };

    // UC実行
    let ent = match uc.get_track(&id.to_string()).await {
        Ok(t) => t,
        Err(_) => {
            // FIXME
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    // レスポンス作成
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

    // return
    HttpResponse::Ok().content_type("text/html").body(resp)
}
