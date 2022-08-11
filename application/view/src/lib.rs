use std::sync::Arc;
use actix_web::{get, web, App, HttpServer, Responder};
use tera::Tera;

// use domain::port::log::Log;
// use domain::port::repository::TrackRepository;

use mock_gateway::TrackGateway;
use text_logger::Logger;
use usecase::track_usecase::TrackUsecase;

mod track;
use track::track_controller;
mod search;
use search::search_controller;
mod modules;

#[get("/")]
async fn healthcheck() -> impl Responder {
    format!("server running!")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let template_file = "application/view/templates/**/*.html"; // FIXME
    let tera = match Tera::new(template_file) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            std::process::exit(1);
        }
    };
    // tera.autoescape_on(vec![]); // disable auto-escaping
    //
    let uc = TrackUsecase {
        repo: Arc::new(TrackGateway::new()),
        log: Arc::new(Logger::new("xxxxxxxx".into())),
    };


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(uc.clone()))
            .service(healthcheck)
            .service(track_controller)
            .service(search_controller)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
