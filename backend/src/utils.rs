use std::fs;
use std::io;
use std::path::Path;
use serde_yaml;
use reqwest;
use pulldown_cmark::{Parser, Options, html, Event, Tag, CodeBlockKind};
use regex::Regex;
use base64::Engine;
use crate::models::*;

/// Input validation error types
#[derive(Debug)]
pub enum ValidationError {
    InvalidCategory(String),
    InvalidSlug(String),
    PathTraversal(String),
    TooLong(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidCategory(msg) => write!(f, "Invalid category: {}", msg),
            ValidationError::InvalidSlug(msg) => write!(f, "Invalid slug: {}", msg),
            ValidationError::PathTraversal(msg) => write!(f, "Path traversal detected: {}", msg),
            ValidationError::TooLong(msg) => write!(f, "Input too long: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validates category parameter
/// Categories must be alphanumeric with hyphens/underscores only
pub fn validate_category(category: &str) -> Result<(), ValidationError> {
    // Check length
    if category.is_empty() {
        return Err(ValidationError::InvalidCategory("Category cannot be empty".to_string()));
    }
    if category.len() > 50 {
        return Err(ValidationError::TooLong("Category name too long".to_string()));
    }
    
    // Check for path traversal
    if category.contains("..") || category.contains('/') || category.contains('\\') {
        return Err(ValidationError::PathTraversal("Category contains path traversal characters".to_string()));
    }
    
    // Only allow alphanumeric, hyphens, and underscores
    let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !valid_chars.is_match(category) {
        return Err(ValidationError::InvalidCategory("Category must contain only letters, numbers, hyphens, and underscores".to_string()));
    }
    
    // Check against whitelist of allowed categories
    let allowed_categories = ["project", "blog", "page"];
    if !allowed_categories.contains(&category) {
        return Err(ValidationError::InvalidCategory(format!("Category '{}' not allowed. Allowed: {:?}", category, allowed_categories)));
    }
    
    Ok(())
}

/// Validates slug parameter
/// Slugs must be URL-safe and prevent path traversal
pub fn validate_slug(slug: &str) -> Result<(), ValidationError> {
    // Check length
    if slug.is_empty() {
        return Err(ValidationError::InvalidSlug("Slug cannot be empty".to_string()));
    }
    if slug.len() > 100 {
        return Err(ValidationError::TooLong("Slug too long".to_string()));
    }
    
    // Check for path traversal
    if slug.contains("..") || slug.contains('/') || slug.contains('\\') {
        return Err(ValidationError::PathTraversal("Slug contains path traversal characters".to_string()));
    }
    
    // Check for null bytes and other dangerous characters
    if slug.contains('\0') || slug.contains('\n') || slug.contains('\r') {
        return Err(ValidationError::InvalidSlug("Slug contains invalid characters".to_string()));
    }
    
    // Only allow URL-safe characters: alphanumeric, hyphens, underscores, and dots
    let valid_chars = regex::Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap();
    if !valid_chars.is_match(slug) {
        return Err(ValidationError::InvalidSlug("Slug must contain only letters, numbers, dots, hyphens, and underscores".to_string()));
    }
    
    // Prevent starting or ending with dots (hidden files)
    if slug.starts_with('.') || slug.ends_with('.') {
        return Err(ValidationError::InvalidSlug("Slug cannot start or end with dots".to_string()));
    }
    
    Ok(())
}

/// Creates a safe file path by validating and joining components
pub fn create_safe_content_path(content_base: &str, category: &str, filename: Option<&str>) -> Result<String, ValidationError> {
    validate_category(category)?;
    
    let mut path = format!("{}/{}", content_base, category);
    
    if let Some(file) = filename {
        validate_slug(file)?;
        path = format!("{}/{}.md", path, file);
    }
    
    // Final safety check: ensure the resolved path stays within content directory
    let canonical_base = std::path::Path::new(content_base).canonicalize()
        .map_err(|_| ValidationError::PathTraversal("Invalid content base path".to_string()))?;
    
    let canonical_target = std::path::Path::new(&path).canonicalize();
    if let Ok(target) = canonical_target {
        if !target.starts_with(&canonical_base) {
            return Err(ValidationError::PathTraversal("Path escapes content directory".to_string()));
        }
    }
    
    Ok(path)
}

pub fn get_content_files(category: &str, content_path: &str) -> Result<Vec<String>, io::Error> {
    let content_dir = format!("{}/{}", content_path, category);
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&content_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "md" {
                            if let Some(filename) = path.file_name() {
                                if let Some(filename_str) = filename.to_str() {
                                    files.push(filename_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(files)
}

pub fn parse_markdown_file(file_path: &str, category: &str) -> Result<ContentItem, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    
    // Extract frontmatter and content
    let (frontmatter, markdown_content) = if content.starts_with("---\n") {
        let end = content[4..].find("\n---\n").unwrap_or(0) + 4;
        let frontmatter_str = &content[4..end];
        let content_str = &content[end + 4..];
        (frontmatter_str.trim(), content_str.trim())
    } else {
        ("", content.trim())
    };
    
    // Parse frontmatter
    let metadata: ContentMetadata = if frontmatter.is_empty() {
        ContentMetadata {
            title: "Untitled".to_string(),
            date: "2024-01-01".to_string(),
            tags: vec![],
            description: "".to_string(),
            image: None,
            feature: None,
        }
    } else {
        serde_yaml::from_str(frontmatter)?
    };
    
    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = Parser::new_ext(markdown_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);
    
    // Get slug from filename
    let slug = Path::new(file_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    Ok(ContentItem {
        slug,
        metadata,
        html_content,
        category: category.to_string(),
    })
}

pub fn load_github_config(content_path: &str) -> Result<GitHubConfig, Box<dyn std::error::Error>> {
    let config_path = format!("{}/github/config.yaml", content_path);
    let config_content = fs::read_to_string(&config_path)?;
    let config: GitHubConfig = serde_yaml::from_str(&config_content)?;
    Ok(config)
}

pub async fn fetch_github_project(repo: &GitHubRepo) -> Result<GitHubProject, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    // Fetch repository info
    let repo_url = format!("https://api.github.com/repos/{}/{}", repo.owner, repo.repo);
    let repo_response = client
        .get(&repo_url)
        .header("User-Agent", "portfolio-backend")
        .send()
        .await?;
    
    if !repo_response.status().is_success() {
        return Err(format!("GitHub API error: {}", repo_response.status()).into());
    }
    
    let repo_data: serde_json::Value = repo_response.json().await?;
    
    // Fetch README
    let readme_url = format!("https://api.github.com/repos/{}/{}/readme", repo.owner, repo.repo);
    let readme_response = client
        .get(&readme_url)
        .header("User-Agent", "portfolio-backend")
        .send()
        .await?;
    
    let readme_html = if readme_response.status().is_success() {
        let readme_data: serde_json::Value = readme_response.json().await?;
        if let Some(content) = readme_data["content"].as_str() {
            // Decode base64 content
            let decoded = base64::engine::general_purpose::STANDARD.decode(content.replace('\n', ""))?;
            let markdown_content = String::from_utf8(decoded)?;
            
            // Process markdown to fix image URLs
            let processed_markdown = process_github_images(&markdown_content, &repo.owner, &repo.repo);
            
            // Convert markdown to HTML
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            options.insert(Options::ENABLE_TABLES);
            options.insert(Options::ENABLE_FOOTNOTES);
            options.insert(Options::ENABLE_TASKLISTS);
            
            let parser = Parser::new_ext(&processed_markdown, options);
            let mut html_content = String::new();
            
            // Process events to add language classes for syntax highlighting
            let events: Vec<Event> = parser.into_iter().map(|event| {
                match event {
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                        let lang_class = if lang.is_empty() {
                            "language-text".to_string()
                        } else {
                            format!("language-{}", lang)
                        };
                        Event::Html(format!("<pre><code class=\"{}\">", lang_class).into())
                    }
                    Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                        Event::Html("</code></pre>".into())
                    }
                    _ => event,
                }
            }).collect();
            
            html::push_html(&mut html_content, events.into_iter());
            
            // Further process HTML to ensure all GitHub images work
            process_github_html_images(&html_content, &repo.owner, &repo.repo)
        } else {
            "README not available".to_string()
        }
    } else {
        "README not available".to_string()
    };
    
    Ok(GitHubProject {
        owner: repo.owner.clone(),
        repo: repo.repo.clone(),
        display_name: repo.display_name.clone(),
        readme_html,
        url: repo_data["html_url"].as_str().unwrap_or("").to_string(),
        stars: repo_data["stargazers_count"].as_u64().unwrap_or(0) as u32,
        forks: repo_data["forks_count"].as_u64().unwrap_or(0) as u32,
        description: repo_data["description"].as_str().map(|s| s.to_string()),
        feature: repo.feature,
        image: repo.image.clone(),
    })
}

fn process_github_images(markdown: &str, owner: &str, repo: &str) -> String {
    use regex::Regex;
    
    // Pattern to match markdown images: ![alt](src)
    let img_regex = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)").unwrap();
    
    img_regex.replace_all(markdown, |caps: &regex::Captures| {
        let alt = &caps[1];
        let src = &caps[2];
        
        let new_src = convert_github_image_url(src, owner, repo);
        format!("![{}]({})", alt, new_src)
    }).to_string()
}

fn process_github_html_images(html: &str, owner: &str, repo: &str) -> String {
    use regex::Regex;
    
    // Pattern to match HTML img tags: <img src="..." /> or <img src="..." >
    let img_regex = Regex::new(r#"<img([^>]*?)src="([^"]*)"([^>]*?)/?>?"#).unwrap();
    
    img_regex.replace_all(html, |caps: &regex::Captures| {
        let before_src = &caps[1];
        let src = &caps[2];
        let after_src = &caps[3];
        
        let new_src = convert_github_image_url(src, owner, repo);
        format!(r#"<img{}src="{}"{}/>"#, before_src, new_src, after_src)
    }).to_string()
}

fn convert_github_image_url(src: &str, owner: &str, repo: &str) -> String {
    // If it's already a full URL, return as is
    if src.starts_with("http://") || src.starts_with("https://") {
        return src.to_string();
    }
    
    // If it's a relative path, convert to GitHub raw URL
    if src.starts_with("./") || src.starts_with("../") || !src.starts_with("/") {
        // Remove leading "./" if present
        let clean_src = src.strip_prefix("./").unwrap_or(src);
        
        // Handle "../" paths by going up directories (simplified - assume they want root)
        let clean_src = if clean_src.starts_with("../") {
            clean_src.strip_prefix("../").unwrap_or(clean_src)
        } else {
            clean_src
        };
        
        return format!("https://raw.githubusercontent.com/{}/{}/main/{}", owner, repo, clean_src);
    }
    
    // If it starts with "/", it's an absolute path in the repo
    if src.starts_with("/") {
        return format!("https://raw.githubusercontent.com/{}/{}/main{}", owner, repo, src);
    }
    
    // Default case - treat as relative to repo root
    format!("https://raw.githubusercontent.com/{}/{}/main/{}", owner, repo, src)
}

// Authentication utilities
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Utc, Duration};
use actix_web::{HttpRequest, HttpResponse, Result, dev::ServiceRequest, dev::ServiceResponse, Error};
use actix_web::dev::{forward_ready, Service, Transform};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};
use std::rc::Rc;

/// JWT Secret key - in production this should be from environment variable
const JWT_SECRET: &[u8] = b"your_super_secret_jwt_key_change_this_in_production";

/// Token expiration time (24 hours)
const TOKEN_EXPIRATION_HOURS: i64 = 24;

/// Create a JWT token for an authenticated user
pub fn create_jwt_token(username: &str, role: &str) -> Result<(String, chrono::DateTime<Utc>), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let expires_at = now + Duration::hours(TOKEN_EXPIRATION_HOURS);
    
    let claims = Claims {
        sub: username.to_owned(),
        exp: expires_at.timestamp(),
        iat: now.timestamp(),
        role: role.to_owned(),
    };
    
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))?;
    Ok((token, expires_at))
}

/// Verify and decode a JWT token
pub fn verify_jwt_token(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET), &validation)?;
    
    // Check if token is expired
    let now = Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err("Token expired".into());
    }
    
    Ok(token_data.claims)
}

/// Hash a password using simple hash (for demo - use proper bcrypt in production)
pub fn hash_password(password: &str) -> String {
    // Simple hash for demo - in production use proper bcrypt
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("hash_{}", hasher.finish())
}

/// Verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> bool {
    hash_password(password) == hash
}

/// Get admin user credentials from environment variables (with fallback for development)
pub fn get_admin_user() -> AuthUser {
    let username = std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let password = std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());
    
    AuthUser {
        username,
        password_hash: hash_password(&password),
        role: "admin".to_string(),
    }
}

/// Extract and verify JWT token from request
pub fn extract_token_from_request(req: &HttpRequest) -> Result<Claims, String> {
    let auth_header = req.headers().get("authorization");
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                
                match verify_jwt_token(token) {
                    Ok(claims) => return Ok(claims),
                    Err(e) => return Err(format!("Invalid token: {}", e)),
                }
            }
        }
    }
    
    Err("Missing or invalid authorization header".to_string())
}

/// Helper function to check admin authentication in handlers
pub fn check_admin_auth(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    match extract_token_from_request(req) {
        Ok(claims) => {
            if claims.role == "admin" {
                Ok(claims)
            } else {
                Err(HttpResponse::Forbidden().json(
                    ApiResponse::<()>::error("Insufficient permissions")
                ))
            }
        }
        Err(_) => {
            Err(HttpResponse::Unauthorized().json(
                ApiResponse::<()>::error("Authentication required")
            ))
        }
    }
}

// File Management utilities

/// Create safe file path for file operations (extends content path validation)
pub fn create_safe_file_path(base_path: &str, relative_path: &str) -> Result<String, ValidationError> {
    // Validate the relative path
    if relative_path.contains("..") || relative_path.contains('\0') {
        return Err(ValidationError::PathTraversal("Path contains dangerous characters".to_string()));
    }
    
    // Remove leading slash if present
    let clean_path = relative_path.trim_start_matches('/');
    
    let full_path = if clean_path.is_empty() {
        base_path.to_string()
    } else {
        format!("{}/{}", base_path, clean_path)
    };
    
    // Final safety check: ensure the resolved path stays within base directory
    let canonical_base = std::path::Path::new(base_path).canonicalize()
        .map_err(|_| ValidationError::PathTraversal("Invalid base path".to_string()))?;
    
    if let Ok(canonical_target) = std::path::Path::new(&full_path).canonicalize() {
        if !canonical_target.starts_with(&canonical_base) {
            return Err(ValidationError::PathTraversal("Path escapes base directory".to_string()));
        }
    }
    
    Ok(full_path)
}

/// List directory contents
pub fn list_directory_contents(dir_path: &str) -> Result<DirectoryContents, Box<dyn std::error::Error>> {
    use std::fs;
    use std::path::Path;
    
    let path = Path::new(dir_path);
    if !path.exists() {
        return Err("Directory does not exist".into());
    }
    
    if !path.is_dir() {
        return Err("Path is not a directory".into());
    }
    
    let mut items = Vec::new();
    let entries = fs::read_dir(path)?;
    
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        let file_path = entry.path();
        let metadata = entry.metadata()?;
        
        // Calculate relative path from base
        let relative_path = file_path.strip_prefix(path)
            .unwrap_or(&file_path)
            .to_string_lossy()
            .to_string();
        
        let file_item = FileItem {
            name: file_name,
            path: relative_path,
            is_directory: metadata.is_dir(),
            size: if metadata.is_file() { Some(metadata.len()) } else { None },
            modified: metadata.modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| {
                    let datetime: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp(duration.as_secs() as i64, 0).unwrap_or_default();
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }),
        };
        
        items.push(file_item);
    }
    
    // Sort items: directories first, then files, both alphabetically
    items.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    
    // Determine parent path
    let parent_path = path.parent()
        .and_then(|p| p.to_str())
        .map(|s| s.to_string());
    
    Ok(DirectoryContents {
        current_path: dir_path.to_string(),
        parent_path,
        items,
    })
}

/// Handle file upload from multipart
pub async fn handle_file_upload(
    payload: &mut actix_multipart::Multipart,
    destination_dir: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    use futures_util::TryStreamExt;
    use std::io::Write;
    
    let mut uploaded_files = Vec::new();
    
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition();
        
        if let Some(filename) = content_disposition.get_filename() {
            let filename_owned = filename.to_string();
            let filepath = std::path::Path::new(destination_dir).join(&filename_owned);
            
            // Create destination directory if it doesn't exist
            if let Some(parent) = filepath.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            // Create file and write chunks
            let mut file = std::fs::File::create(&filepath)?;
            while let Some(chunk) = field.try_next().await? {
                file.write_all(&chunk)?;
            }
            
            uploaded_files.push(filename_owned);
        }
    }
    
    if uploaded_files.is_empty() {
        return Err("No files found in upload".into());
    }
    
    if uploaded_files.len() == 1 {
        Ok(uploaded_files[0].clone())
    } else {
        Ok(format!("{} files", uploaded_files.len()))
    }
}

/// Delete file or folder
pub fn delete_file_or_folder(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = std::path::Path::new(path);
    
    if !path.exists() {
        return Err("File or folder does not exist".into());
    }
    
    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }
    
    Ok(())
}

/// Rename file or folder
pub fn rename_file_or_folder(old_path: &str, new_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let old = std::path::Path::new(old_path);
    let new = std::path::Path::new(new_path);
    
    if !old.exists() {
        return Err("Source file or folder does not exist".into());
    }
    
    if new.exists() {
        return Err("Destination already exists".into());
    }
    
    std::fs::rename(old, new)?;
    Ok(())
}

/// Move file or folder
pub fn move_file_or_folder(source_path: &str, destination_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = std::path::Path::new(source_path);
    let destination = std::path::Path::new(destination_path);
    
    if !source.exists() {
        return Err("Source file or folder does not exist".into());
    }
    
    // Create destination directory if it doesn't exist
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    if destination.exists() {
        return Err("Destination already exists".into());
    }
    
    std::fs::rename(source, destination)?;
    Ok(())
}

/// Create directory
pub fn create_directory(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path)?;
    Ok(())
}

/// Serve file for download
pub fn serve_file_download(file_path: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Read;
    
    let path = std::path::Path::new(file_path);
    
    if !path.exists() || !path.is_file() {
        return Err("File does not exist".into());
    }
    
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    
    let filename = path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download");
    
    Ok(HttpResponse::Ok()
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", filename)))
        .append_header(("Content-Type", "application/octet-stream"))
        .body(contents))
}