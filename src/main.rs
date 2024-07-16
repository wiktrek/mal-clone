use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

async fn index() -> impl Responder {
    "Hello world!"
}

#[derive(Deserialize)]
struct NewAnime {
    id: u32,
    title: String,
}
#[post("/post")]
async fn post(anime: web::Json<NewAnime>) -> Result<String> {
    Ok(format!("Anime submitted: {}",anime.title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service((
            web::scope("/app")
                .route("/", web::get().to(index)),
            web::scope("/api")
                .service(post)
            )
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}