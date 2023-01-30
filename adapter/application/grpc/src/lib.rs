use tonic::{transport::Server, Request, Response, Status};

use port::Container;

use tracksearch::track_server::{Track, TrackServer};
use tracksearch::{GetTrackReply, GetTrackRequest, SearchTrackReply, SearchTrackRequest};

pub mod tracksearch {
    tonic::include_proto!("tracksearch"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyTrack {}

#[tonic::async_trait]
impl Track for MyTrack {
    async fn get_track(
        &self,
        request: Request<GetTrackRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<GetTrackReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = GetTrackReply {
            id: "aaaaa".to_string(),
            title: "aaaaa".to_string(),
            artist: "aaaaa".to_string(),
            image_url: "aaaaa".to_string(),
            popularity: 34,
            danceability: 34,
            energy: 34,
            valence: 34,
            acousticness: 34,
            instrumentalness: 34,
            liveness: 34,
            speechiness: 34,
            time: 340,
            bpm: 120,
            key: 3,
            mode: 2,
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn search_track(
        &self,
        request: Request<SearchTrackRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<SearchTrackReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = SearchTrackReply { tracks: vec![] };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
pub async fn serve(port: u16, _: Container) -> anyhow::Result<()> {
    let addr = (String::from("[::1]:") + &port.to_string()).parse()?; // "[::1]:8080"
    let greeter = MyTrack::default();

    Server::builder()
        .add_service(TrackServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
