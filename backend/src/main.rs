use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_files as fs;
use std::collections::HashMap;
use std::sync::Mutex;
use portfolio_backend::*;

#[derive(Clone)]
pub struct AppState {
    pub github_cache: web::Data<Mutex<HashMap<String, CachedGithubProject>>>,
    pub content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
    pub config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = AppConfig::from_env().map_err(|e| {
        eprintln!("Configuration error: {}", e);
        std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
    })?;

    let github_cache = web::Data::new(Mutex::new(HashMap::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::new()));

    let app_state = AppState {
        github_cache: github_cache.clone(),
        content_cache: content_cache.clone(),
        config: config.clone(),
    };

    let bind_addr = format!("{}:{}", config.host, config.port);
    println!("Starting portfolio server on http://{}", bind_addr);

    let config_clone = config.clone();
    HttpServer::new(move || {
        App::new()
            .app_data(github_cache.clone())
            .app_data(content_cache.clone())
            .app_data(web::Data::new(config_clone.clone()))
            .wrap(Logger::default())
            .wrap(
                actix_web::middleware::DefaultHeaders::new()
                    .add(("Access-Control-Allow-Origin", config_clone.frontend_url.as_str()))
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
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/verify", web::get().to(verify_token))
                            .route("/logout", web::post().to(logout))
                    )
                    .service(
                        web::scope("/admin")
                            .route("/refresh-github", web::post().to(refresh_github_cache))
                            .service(
                                web::scope("/files")
                                    .route("/list/{path:.*}", web::get().to(list_files))
                                    .route("/upload/{path:.*}", web::post().to(upload_file))
                                    .route("/download/{path:.*}", web::get().to(download_file))
                                    .route("/read/{path:.*}", web::get().to(read_file_content))
                                    .route("/save/{path:.*}", web::post().to(save_file_content))
                                    .route("/delete", web::post().to(delete_file))
                                    .route("/rename", web::post().to(rename_file))
                                    .route("/move", web::post().to(move_file))
                                    .route("/create-folder", web::post().to(create_folder))
                            )
                    )
            )
            .service(fs::Files::new("/", &config_clone.frontend_path).index_file("index.html"))
    })
    .bind(&bind_addr)?
    .run()
    .await
}