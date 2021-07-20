use super::Container;
use crate::entity::track::{TrackEntity, TrackId};
use crate::usecase::track_usecase::TrackUsecase;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use std::convert::TryFrom; // FIXME: ここの参照は問題ないか確認

#[derive(Template)]
#[template(path = "track.html")]
struct TrackTemplate {
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

impl TrackTemplate {
    fn from(e: TrackEntity) -> Self {
        return TrackTemplate {
            id: e.id.to_string(),
            title: e.title.to_string(),
            artist: e.artist.to_string(),
            image_url: e.image.to_string(),
            popularity: e.analysis.popularity().to_string(),
            danceability: e.analysis.danceability().to_string(),
            energy: e.analysis.energy().to_string(),
            valence: e.analysis.valence().to_string(),
            acousticness: e.analysis.acousticness().to_string(),
            instrumentalness: e.analysis.instrumentalness().to_string(),
            liveness: e.analysis.liveness().to_string(),
            speechiness: e.analysis.speechiness().to_string(),
            time: "4:12".to_string(),
            bpm: "83.9".to_string(),
            key: "C".to_string(),
            mode: "Major".to_string(),
        };
    }
}

#[get("/track/{id}")]
pub async fn track_controller(
    data: web::Data<Container>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = match TrackId::try_from(path.into_inner().0) {
        Ok(o) => o,
        Err(_) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body("IDが不正です"))
        }
    };
    let uc = TrackUsecase {
        repo: data.track_repository,
        log: data.logger,
    };
    let entity = uc.get_track(id).await;
    let html = match entity {
        Ok(entity) => TrackTemplate::from(entity),
        // FIXME! FIXME! FIXME!
        Err(_) => {
            return Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body("Server Error!"))
        }
    };
    let responce_body = html.render().unwrap(); // FIXME: ここunwrapではなくてちゃんとハンドリング

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(responce_body))
}
