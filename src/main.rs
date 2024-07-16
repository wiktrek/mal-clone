use std::fs;
use std::path::Path;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::*;
async fn index() -> impl Responder {
    "Hello world!"
}

#[derive(Deserialize, Serialize)]
struct NewAnime {
    id: u32,
    title: String,
    description: String,
}
#[post("/post")]
async fn post(anime: web::Json<NewAnime>) -> Result<String> {
    create_file(NewAnime {
        id: anime.id,
        title: anime.title.clone(),
        description: anime.description.clone(),
    }).await;
    Ok(format!("Anime submitted: {} \nid: {}", anime.title, anime.id))
}
async fn create_file(anime: NewAnime) {
    
    if !Path::new("./anime").exists() {
        let _ = fs::create_dir("./anime");   
    }
    let _ = fs::write(format!("./anime/{}.json", anime.id), serde_json::to_string(&anime).unwrap());
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