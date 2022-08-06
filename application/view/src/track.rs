use std::convert::From;

use domain::model::track_entity::TrackEntity;
use usecase::track_usecase::TrackUsecase;

// FIXME
use mock_gateway::TrackGateway;
use text_logger::Logger;

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
            time: "4:12".to_string(),  // FIXME
            bpm: "83.9".to_string(),   // FIXME
            key: "C".to_string(),      // FIXME
            mode: "Major".to_string(), // FIXME
        }
    }
}

#[get("/track/{id}")]
async fn track_controller(id: web::Path<String>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();

    let uc = TrackUsecase {
        repo: TrackGateway::new(),
        log: Logger::new("xxxxxxxx".into()),
    };

    let ent = match uc.get_track(id.to_string()).await {
        Ok(t) => t,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

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

    HttpResponse::Ok().content_type("text/html").body(resp)
}
