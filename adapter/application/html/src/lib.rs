use actix_web::{get, web, App, HttpServer, Responder};
use tera::Tera;

use port::Container;

mod track;
use track::track_controller;
mod search;
use search::search_controller;

#[get("/.genki")]
async fn healthcheck() -> impl Responder {
    "server running!".to_string()
}

#[actix_web::main]
// async fn main(port: u16, container: Container) -> std::io::Result<()> {
pub async fn serve(port: u16, container: Container) -> anyhow::Result<()> {
    let template_file = "adapter/application/html/templates/**/*.html"; // FIXME
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
            .app_data(web::Data::new(container.clone()))
            .service(healthcheck)
            .service(track_controller)
            .service(search_controller)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
