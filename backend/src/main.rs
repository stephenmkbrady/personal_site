use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use actix_files as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

mod handlers;
mod models;
mod utils;

use handlers::*;
use models::*;

#[derive(Clone)]
pub struct AppState {
    pub github_cache: web::Data<Mutex<HashMap<String, CachedGithubProject>>>,
    pub content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let github_cache = web::Data::new(Mutex::new(HashMap::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::new()));

    let app_state = AppState {
        github_cache: github_cache.clone(),
        content_cache: content_cache.clone(),
    };

    println!("Starting portfolio server on http://localhost:4000");

    HttpServer::new(move || {
        App::new()
            .app_data(github_cache.clone())
            .app_data(content_cache.clone())
            .wrap(Logger::default())
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", "*"))
                    .add(("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"))
                    .add(("Access-Control-Allow-Headers", "Content-Type, Authorization"))
            )
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    .service(
                        web::scope("/content")
                            .route("/{category}", web::get().to(get_content_list))
                            .route("/{category}/{slug}", web::get().to(get_content_item))
                            .route("/tags", web::get().to(get_content_tags))
                    )
                    .service(
                        web::scope("/github")
                            .route("/projects", web::get().to(get_github_projects))
                    )
                    .service(
                        web::scope("/admin")
                            .route("/refresh-github", web::post().to(refresh_github_cache))
                    )
            )
            .service(fs::Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}