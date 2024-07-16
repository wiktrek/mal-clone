use std::fs;
use std::path::Path;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use serde_json::*;
async fn index() -> impl Responder {
    "Hello world!"   
}
#[derive(Deserialize, Serialize, Clone)]
struct Episodes {
    all_episodes: u32,
    watched: u32,
}
#[derive(Deserialize, Serialize, Clone)]
struct NewAnime {
    user_id: u32,
    id: u32,
    title: String,
    description: String,
    rating: u32,
    episodes_watched: Episodes,
}
#[post("/post")]
async fn post(anime: web::Json<NewAnime>) -> Result<String> {
    create_file(NewAnime {
        id: anime.id.clone(),
        title: anime.title.clone(),
        description: anime.description.clone(),
        rating: anime.rating.clone(),
        episodes_watched: anime.episodes_watched.clone(),
        user_id: anime.user_id.clone(),
    }).await;
    Ok(format!("Anime submitted: {} \nid: {}", anime.title, anime.id))
}
async fn create_file(anime: NewAnime) {
    
    if !Path::new("./anime").exists() {
        let _ = fs::create_dir("./anime");   
    }
    if !Path::new(&format!("./anime/{}.json", anime.user_id)).exists() {
        let _ = fs::write(format!("./anime/{}.json", anime.user_id), format!("[{}]",serde_json::to_string(&anime).unwrap()));
    }
    let text = fs::read_to_string(&format!("./anime/{}.json", anime.user_id)).unwrap();
    let mut json: Vec<NewAnime> = serde_json::from_str(&text).expect("Error 'string -> json'");
    json.push(anime.clone());
    fs::write(&format!("./anime/{}.json", anime.user_id), serde_json::to_string(&json).unwrap());
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