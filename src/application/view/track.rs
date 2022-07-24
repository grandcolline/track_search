use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct Track {
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

#[get("/track/{id}")]
async fn track_controller(id: web::Path<String>, tera: web::Data<Tera>) -> impl Responder {
    let mut context = Context::new();
    context.insert(
        "track",
        &Track {
            id: id.to_string(),
            title: "クロノスタシス".to_string(),
            artist: "きのこ帝国".to_string(),
            image_url: "https://i.scdn.co/image/ab67616d00001e02963cf0d3369083bc68e80141"
                .to_string(),
            popularity: 58.to_string(),
            danceability: 46.to_string(),
            energy: 75.to_string(),
            valence: 70.to_string(),
            acousticness: 3.to_string(),
            instrumentalness: 1.to_string(),
            liveness: 13.to_string(),
            speechiness: 3.to_string(),
            time: "4:12".to_string(),
            bpm: "83.9".to_string(),
            key: "C".to_string(),
            mode: "Major".to_string(),
        },
    );

    let resp = match tera.render("track.html", &context) {
        Ok(t) => t,
        Err(e) => {
            println!("error: {}", e);
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("Server Error");
        }
    };

    HttpResponse::Ok().content_type("text/html").body(resp)
}
