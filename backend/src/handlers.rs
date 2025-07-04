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



