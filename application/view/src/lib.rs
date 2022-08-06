use actix_web::{get, web, App, HttpServer, Responder};
use tera::Tera;

// use domain::port::log::Log;
// use domain::port::repository::TrackRepository;

mod track;
use track::track_controller;
mod search;
use search::search_controller;

// pub struct Modules {
//   track_repository: TrackRepository,
//   log: Log,
// }


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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(healthcheck)
            .service(track_controller)
            .service(search_controller)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
