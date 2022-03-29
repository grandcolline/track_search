use crate::adapter::gateway::mock::track_gateway::TrackGateway; // FIXME: ここでこれを参照するのはマジでクソ。子宮から人生やり直すレベル。
use crate::adapter::logger::text::logger::Logger; // FIXME: ここでこれを参照するのはマジでクソ。子宮から人生やり直すレベル。
use actix_web::{web, App, HttpServer};

// use actix_web::middleware::Logger;
pub mod systems;
pub mod track;

// FIXME: ここの実装はdriver層に移動させる？
#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Container {
                track_port: TrackGateway {},
                logger: Logger::new("xxxxxxxxxx".to_string()),
            })
            // .wrap(Logger::default())
            .configure(routes)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

#[allow(dead_code)]
fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/health").route(web::get().to(systems::health)))
        .service(web::resource("/track").route(web::get().to(track::track)));
}

pub struct Container {
    track_port: TrackGateway,
    logger: Logger,
}
