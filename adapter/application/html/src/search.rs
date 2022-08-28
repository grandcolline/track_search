use std::convert::From;

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use entity::track_dto::TrackDto;
use port::Container;
use usecase::track_usecase::TrackUsecase;

#[derive(Serialize)]
struct Search {
    keyword: String,
}

#[derive(Serialize)]
struct Track {
    id: String,
    name: String,
    image: String,
    artist: String,
}

impl From<TrackDto> for Track {
    fn from(dto: TrackDto) -> Self {
        Self {
            id: dto.id,
            name: dto.title,
            image: dto.image,
            artist: dto.artist,
        }
    }
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[get("/search")]
async fn search_controller(
    query: web::Query<SearchQuery>,
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
    let dtos = match uc.search_track(query.q.to_string()).await {
        Ok(v) => v,
        Err(_) => {
            // FIXME
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    // response作成
    context.insert("keyword", &query.q.to_string());

    let mut tracks = vec![];
    for dto in dtos.iter() {
        tracks.push(Track::from(dto.clone()));
    }
    context.insert("tracks", &tracks);

    let resp = match tera.render("search.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("error: {}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    // response
    HttpResponse::Ok().content_type("text/html").body(resp)
}
