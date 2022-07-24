use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use tera::{Context, Tera};

mod track;
use track::track_controller;

#[derive(Serialize)]
pub struct Greet {
    name: String,
}

#[get("/")]
async fn healthcheck() -> impl Responder {
    format!("server running!")
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let tera = match Tera::new("templates/**/*.html") {
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
