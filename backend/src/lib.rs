use std::env;

pub mod handlers;
pub mod models;
pub mod utils;

pub use handlers::*;
pub use models::*;
pub use utils::*;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub content_path: String,
    pub frontend_path: String,
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "4000".to_string())
            .parse()
            .map_err(|_| "Invalid PORT value")?;
        let content_path = env::var("CONTENT_PATH").unwrap_or_else(|_| "../content".to_string());
        let frontend_path = env::var("FRONTEND_PATH").unwrap_or_else(|_| "../frontend".to_string());
        let frontend_url = env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        Ok(AppConfig {
            host,
            port,
            content_path,
            frontend_path,
            frontend_url,
        })
    }
}