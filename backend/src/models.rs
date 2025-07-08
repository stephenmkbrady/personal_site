use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMetadata {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub description: String,
    pub image: Option<String>,
    pub feature: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub slug: String,
    pub metadata: ContentMetadata,
    pub html_content: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedContent {
    pub content: ContentItem,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub owner: String,
    pub repo: String,
    pub display_name: String,
    pub feature: Option<bool>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProject {
    pub owner: String,
    pub repo: String,
    pub display_name: String,
    pub readme_html: String,
    pub url: String,
    pub stars: u32,
    pub forks: u32,
    pub description: Option<String>,
    pub feature: Option<bool>,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedGithubProject {
    pub project: GitHubProject,
    pub cached_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub repositories: Vec<GitHubRepo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
}

// Authentication models
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // username
    pub exp: i64,     // expiration timestamp
    pub iat: i64,     // issued at timestamp
    pub role: String, // user role (admin)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}

// File Management models
#[derive(Debug, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryContents {
    pub current_path: String,
    pub parent_path: Option<String>,
    pub items: Vec<FileItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperationRequest {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRenameRequest {
    pub old_path: String,
    pub new_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMoveRequest {
    pub source_path: String,
    pub destination_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContentRequest {
    pub content: String,
}

