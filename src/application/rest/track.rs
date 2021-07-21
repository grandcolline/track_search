use crate::application::rest::Container;
use crate::entity::track::{TrackEntity, TrackId};
use crate::usecase::track_usecase::TrackUsecase;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

pub async fn track(data: web::Data<Container>) -> impl Responder {
    // FIXME
    let id = match TrackId::try_from("aaaaaaaaaaaaaaa".to_string()) {
        Ok(id) => id,
        Err(_) => return HttpResponse::InternalServerError().json(""),
    };
    let uc = TrackUsecase {
        repo: data.track_port,
        log: data.logger,
    };
    let track_entity = uc.get_track(id).await;
    match track_entity {
        Ok(track_entity) => HttpResponse::Ok().json(TrackJson::from(track_entity)),
        // FIXME: error handling
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TrackJson {
    id: String,
    title: String,
}

impl TrackJson {
    fn from(e: TrackEntity) -> Self {
        TrackJson {
            id: e.id.to_string(),
            title: e.title.to_string(),
        }
    }
}
