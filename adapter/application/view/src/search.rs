use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Serialize)]
struct Search {
    keyword: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[get("/search")]
async fn search_controller(
    query: web::Query<SearchQuery>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let mut context = Context::new();

    // バリデーション

    // UC作成

    // UC実行

    // response作成
    context.insert(
        "search",
        &Search {
            keyword: query.q.to_string(),
        },
    );
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
