use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::models::*;
use crate::utils::*;

pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(ApiResponse::success("Server is running")))
}

pub async fn get_content_list(
    path: web::Path<String>,
    content_cache: web::Data<Mutex<HashMap<String, CachedContent>>>,
) -> Result<HttpResponse> {
    let category = path.into_inner();
    
    match get_content_files(&category) {
        Ok(files) => {
            let mut content_items = Vec::new();
            
            for file in files {
                let file_path = format!("../content/{}/{}", category, file);
                match parse_markdown_file(&file_path, &category) {
                    Ok(content) => content_items.push(content),
                    Err(e) => {
                        eprintln!("Error parsing {}: {}", file_path, e);
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
) -> Result<HttpResponse> {
    let (category, slug) = path.into_inner();
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
    
    // Load from file
    let file_path = format!("../content/{}/{}.md", category, slug);
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
) -> Result<HttpResponse> {
    let mut all_tags = std::collections::HashSet::new();
    
    // Get tags from all categories
    for category in &["project", "blog"] {
        if let Ok(files) = get_content_files(category) {
            for file in files {
                let file_path = format!("../content/{}/{}", category, file);
                if let Ok(content) = parse_markdown_file(&file_path, category) {
                    for tag in content.metadata.tags {
                        all_tags.insert(tag);
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
) -> Result<HttpResponse> {
    // Load GitHub config
    let config = match load_github_config() {
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
    github_cache: web::Data<Mutex<HashMap<String, CachedGithubProject>>>,
) -> Result<HttpResponse> {
    // Clear the cache
    {
        let mut cache = github_cache.lock().unwrap();
        cache.clear();
    }
    
    Ok(HttpResponse::Ok().json(ApiResponse::success("GitHub cache refreshed")))
}



