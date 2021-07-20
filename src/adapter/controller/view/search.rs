use super::Container;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use serde::Deserialize; // FIXME: ここの参照は問題ないか確認

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    keyword: String,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    q: Option<String>,
}

#[get("/search")]
pub async fn search_consroller(
    _data: web::Data<Container>,
    req: web::Query<SearchRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let keyword = match &req.q {
        None => "".to_string(),
        Some(s) => s.to_string(), // FIXME: ここto_string()していいのか？
    };
    let html = SearchTemplate { keyword };
    let responce_body = html.render().unwrap(); // FIXME: ここunwrapではなくてちゃんとハンドリング

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(responce_body))
}
