use crate::adapter::gateway::mock::track_gateway::TrackGateway; // FIXME: ここでこれを参照するのはマジでクソ。子宮から人生やり直すレベル。
use crate::adapter::logger::text::logger::Logger; // FIXME: ここでこれを参照するのはマジでクソ。子宮から人生やり直すレベル。

// use crate::usecase::repository::track_repository::TrackRepository;
mod search;
mod track;
use actix_web::{App, HttpServer};
use search::search_consroller;
use track::track_controller;

// FIXME: ここの関数はdriver層に移動させる？
#[actix_rt::main]
pub async fn build() -> Result<(), actix_web::Error> {
    // TODO: moveを確認
    HttpServer::new(move || {
        App::new()
            .data(Container {
                track_repository: TrackGateway {},
                logger: Logger {},
            })
            .service(search_consroller)
            .service(track_controller)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await?;
    Ok(())
}

// pub struct RepositoryContainer<T>
// where
//     T: TrackRepository,
// {
//     track_repository: T,
// }

pub struct Container {
    track_repository: TrackGateway,
    logger: Logger,
}
