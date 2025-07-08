use actix_web::{web, HttpResponse, Result, HttpRequest};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::*;
use crate::utils::*;
use crate::AppConfig;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("Server is running")))
}

pub async fn get_content_list(
    path: web::Path<String>,
    content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let category = path.into_inner();
    
    // Validate category parameter
    if let Err(validation_error) = validate_category(&category) {
        return Ok(HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(&format!("Invalid category parameter: {}", validation_error))
        ));
    }
    
    match get_content_files(&category, &app_config.content_path) {
        Ok(files) => {
            let mut content_items = Vec::new();
            
            for file in files {
                // Use safe path creation for additional security
                let file_stem = file.trim_end_matches(".md");
                match create_safe_content_path(&app_config.content_path, &category, Some(file_stem)) {
                    Ok(file_path) => {
                        match parse_markdown_file(&file_path, &category) {
                            Ok(content) => content_items.push(content),
                            Err(e) => {
                                eprintln!("Error parsing {}: {}", file_path, e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Invalid file path for {}: {}", file, e);
                        continue;
                    }
                }
            }
            
            // Sort by date (newest first)
            content_items.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(content_items)))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(ApiResponse::<()>::error(&format!("Error reading content: {}", e))))
        }
    }
}

pub async fn get_content_item(
    path: web::Path<(String, String)>,
    content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let (category, slug) = path.into_inner();
    
    // Validate category and slug parameters
    if let Err(validation_error) = validate_category(&category) {
        return Ok(HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(&format!("Invalid category parameter: {}", validation_error))
        ));
    }
    
    if let Err(validation_error) = validate_slug(&slug) {
        return Ok(HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(&format!("Invalid slug parameter: {}", validation_error))
        ));
    }
    
    let cache_key = format!("{}/{}", category, slug);
    
    // Check cache first
    {
        let cache = content_cache.lock().unwrap();
        if let Some(cached_content) = cache.get(&cache_key) {
            // Check if cache is still valid (1 hour)
            if Utc::now() - cached_content.cached_at < Duration::hours(1) {
                return Ok(HttpResponse::Ok().json(ApiResponse::success(&cached_content.content)));
            }
        }
    }
    
    // Create safe file path
    let file_path = match create_safe_content_path(&app_config.content_path, &category, Some(&slug)) {
        Ok(path) => path,
        Err(validation_error) => {
            return Ok(HttpResponse::BadRequest().json(
                ApiResponse::<()>::error(&format!("Invalid file path: {}", validation_error))
            ));
        }
    };
    
    match parse_markdown_file(&file_path, &category) {
        Ok(content) => {
            // Update cache
            let cached_content = CachedContent {
                content: content.clone(),
                cached_at: Utc::now(),
            };
            
            {
                let mut cache = content_cache.lock().unwrap();
                cache.insert(cache_key, cached_content);
            }
            
            Ok(HttpResponse::Ok().json(ApiResponse::success(content)))
        }
        Err(e) => {
            Ok(HttpResponse::NotFound().json(ApiResponse::<()>::error(&format!("Content not found: {}", e))))
        }
    }
}

pub async fn get_content_tags(
    content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    let mut all_tags = std::collections::HashSet::new();
    
    // Get tags from all categories
    for category in &["project", "blog"] {
        // Validate each category (should be safe since we control the list, but good practice)
        if validate_category(category).is_err() {
            continue;
        }
        
        if let Ok(files) = get_content_files(category, &app_config.content_path) {
            for file in files {
                let file_stem = file.trim_end_matches(".md");
                if let Ok(file_path) = create_safe_content_path(&app_config.content_path, category, Some(file_stem)) {
                    if let Ok(content) = parse_markdown_file(&file_path, category) {
                        for tag in content.metadata.tags {
                            all_tags.insert(tag);
                        }
                    }
                }
            }
        }
    }
    
    let mut tags: Vec<String> = all_tags.into_iter().collect();
    tags.sort();
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(tags)))
}

pub async fn get_github_projects(
    github_cache: web::Data<Mutex<HashMap<String, CachedGithubProject>>>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Load GitHub config
    let config = match load_github_config(&app_config.content_path) {
        Ok(config) => config,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(&format!("Failed to load GitHub config: {}", e))
            ));
        }
    };
    
    let mut projects = Vec::new();
    
    for repo in config.repositories {
        let cache_key = format!("{}/{}", repo.owner, repo.repo);
        
        // Check cache first
        let cached_project = {
            let cache = github_cache.lock().unwrap();
            cache.get(&cache_key).cloned()
        };
        
        if let Some(cached) = cached_project {
            // Check if cache is still valid (24 hours)
            if Utc::now() - cached.cached_at < Duration::hours(24) {
                let mut project = cached.project;
                // Add feature and image properties from config
                project.feature = repo.feature;
                project.image = repo.image.clone();
                projects.push(project);
                continue;
            }
        }
        
        // Fetch from GitHub API
        match fetch_github_project(&repo).await {
            Ok(mut project) => {
                // Add feature and image properties from config
                project.feature = repo.feature;
                project.image = repo.image.clone();
                
                // Cache the project
                let cached_project = CachedGithubProject {
                    project: project.clone(),
                    cached_at: Utc::now(),
                };
                
                {
                    let mut cache = github_cache.lock().unwrap();
                    cache.insert(cache_key, cached_project);
                }
                
                projects.push(project);
            }
            Err(e) => {
                eprintln!("Failed to fetch GitHub project {}/{}: {}", repo.owner, repo.repo, e);
                
                // Create minimal project from config if API fails
                let project = GitHubProject {
                    owner: repo.owner.clone(),
                    repo: repo.repo.clone(),
                    display_name: repo.display_name.clone(),
                    feature: repo.feature,
                    image: repo.image.clone(),
                    readme_html: "README not available".to_string(),
                    url: format!("https://github.com/{}/{}", repo.owner, repo.repo),
                    stars: 0,
                    forks: 0,
                    description: Some("GitHub repository".to_string()),
                };
                projects.push(project);
            }
        }
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success(projects)))
}

pub async fn refresh_github_cache(
    req: HttpRequest,
    github_cache: web::Data<Mutex<HashMap<String, CachedGithubProject>>>,
    _app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            // Clear the cache
            {
                let mut cache = github_cache.lock().unwrap();
                cache.clear();
            }
            
            Ok(HttpResponse::Ok().json(ApiResponse::success("GitHub cache refreshed")))
        }
        Err(response) => Ok(response),
    }
}

// Authentication handlers

/// Login endpoint - validates credentials and returns JWT token
pub async fn login(
    login_request: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    let admin_user = get_admin_user();
    
    // Validate credentials
    if login_request.username != admin_user.username {
        return Ok(HttpResponse::Unauthorized().json(
            ApiResponse::<()>::error("Invalid username or password")
        ));
    }
    
    if !verify_password(&login_request.password, &admin_user.password_hash) {
        return Ok(HttpResponse::Unauthorized().json(
            ApiResponse::<()>::error("Invalid username or password")
        ));
    }
    
    // Create JWT token
    match create_jwt_token(&admin_user.username, &admin_user.role) {
        Ok((token, expires_at)) => {
            let response = LoginResponse {
                token,
                expires_at,
            };
            Ok(HttpResponse::Ok().json(ApiResponse::success(response)))
        }
        Err(e) => {
            eprintln!("JWT creation error: {}", e);
            Ok(HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error("Failed to create authentication token")
            ))
        }
    }
}

/// Verify token endpoint - checks if provided JWT token is valid
pub async fn verify_token(
    req: actix_web::HttpRequest,
) -> Result<HttpResponse> {
    let auth_header = req.headers().get("authorization");
    
    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Remove "Bearer " prefix
                
                match verify_jwt_token(token) {
                    Ok(claims) => {
                        return Ok(HttpResponse::Ok().json(ApiResponse::success(claims)));
                    }
                    Err(e) => {
                        eprintln!("Token verification error: {}", e);
                        return Ok(HttpResponse::Unauthorized().json(
                            ApiResponse::<()>::error("Invalid or expired token")
                        ));
                    }
                }
            }
        }
    }
    
    Ok(HttpResponse::Unauthorized().json(
        ApiResponse::<()>::error("Missing or invalid authorization header")
    ))
}

/// Logout endpoint - client-side token invalidation (server just confirms)
pub async fn logout() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("Logged out successfully")))
}

// File Management handlers

/// List files and folders in a directory
pub async fn list_files(
    req: HttpRequest,
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            let folder_path = path.into_inner();
            
            // Validate and create safe path
            match create_safe_file_path(&app_config.content_path, &folder_path) {
                Ok(safe_path) => {
                    match list_directory_contents(&safe_path) {
                        Ok(contents) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success(contents)))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Failed to list directory: {}", e))
                            ))
                        }
                    }
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error(&format!("Invalid path: {}", e))
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Upload file to directory
pub async fn upload_file(
    req: HttpRequest,
    mut payload: actix_multipart::Multipart,
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            let folder_path = path.into_inner();
            
            // Validate and create safe path
            match create_safe_file_path(&app_config.content_path, &folder_path) {
                Ok(safe_path) => {
                    match handle_file_upload(&mut payload, &safe_path).await {
                        Ok(filename) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success(format!("File '{}' uploaded successfully", filename))))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Upload failed: {}", e))
                            ))
                        }
                    }
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error(&format!("Invalid path: {}", e))
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Delete file or folder
pub async fn delete_file(
    req: HttpRequest,
    delete_request: web::Json<FileOperationRequest>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            match create_safe_file_path(&app_config.content_path, &delete_request.path) {
                Ok(safe_path) => {
                    match delete_file_or_folder(&safe_path) {
                        Ok(_) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success("File/folder deleted successfully")))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Delete failed: {}", e))
                            ))
                        }
                    }
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error(&format!("Invalid path: {}", e))
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Rename file or folder
pub async fn rename_file(
    req: HttpRequest,
    rename_request: web::Json<FileRenameRequest>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            match (
                create_safe_file_path(&app_config.content_path, &rename_request.old_path),
                create_safe_file_path(&app_config.content_path, &rename_request.new_path)
            ) {
                (Ok(old_safe_path), Ok(new_safe_path)) => {
                    match rename_file_or_folder(&old_safe_path, &new_safe_path) {
                        Ok(_) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success("File/folder renamed successfully")))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Rename failed: {}", e))
                            ))
                        }
                    }
                }
                _ => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error("Invalid file paths")
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Move file or folder
pub async fn move_file(
    req: HttpRequest,
    move_request: web::Json<FileMoveRequest>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            match (
                create_safe_file_path(&app_config.content_path, &move_request.source_path),
                create_safe_file_path(&app_config.content_path, &move_request.destination_path)
            ) {
                (Ok(source_safe_path), Ok(dest_safe_path)) => {
                    match move_file_or_folder(&source_safe_path, &dest_safe_path) {
                        Ok(_) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success("File/folder moved successfully")))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Move failed: {}", e))
                            ))
                        }
                    }
                }
                _ => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error("Invalid file paths")
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Create new folder
pub async fn create_folder(
    req: HttpRequest,
    folder_request: web::Json<FileOperationRequest>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            match create_safe_file_path(&app_config.content_path, &folder_request.path) {
                Ok(safe_path) => {
                    match create_directory(&safe_path) {
                        Ok(_) => {
                            Ok(HttpResponse::Ok().json(ApiResponse::success("Folder created successfully")))
                        }
                        Err(e) => {
                            Ok(HttpResponse::InternalServerError().json(
                                ApiResponse::<()>::error(&format!("Folder creation failed: {}", e))
                            ))
                        }
                    }
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error(&format!("Invalid path: {}", e))
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}

/// Download file
pub async fn download_file(
    req: HttpRequest,
    path: web::Path<String>,
    app_config: web::Data<AppConfig>,
) -> Result<HttpResponse> {
    // Check admin authentication
    match check_admin_auth(&req) {
        Ok(_) => {
            let file_path = path.into_inner();
            
            match create_safe_file_path(&app_config.content_path, &file_path) {
                Ok(safe_path) => {
                    match serve_file_download(&safe_path) {
                        Ok(response) => Ok(response),
                        Err(e) => {
                            Ok(HttpResponse::NotFound().json(
                                ApiResponse::<()>::error(&format!("File not found: {}", e))
                            ))
                        }
                    }
                }
                Err(e) => {
                    Ok(HttpResponse::BadRequest().json(
                        ApiResponse::<()>::error(&format!("Invalid path: {}", e))
                    ))
                }
            }
        }
        Err(response) => Ok(response),
    }
}



