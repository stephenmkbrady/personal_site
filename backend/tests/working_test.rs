use actix_web::{test, web, App, http::StatusCode};
use portfolio_backend::*;
use std::collections::HashMap;
use std::sync::Mutex;

fn create_test_config() -> AppConfig {
    AppConfig {
        host: "127.0.0.1".to_string(),
        port: 4000,
        content_path: "../content".to_string(),
        frontend_path: "../frontend".to_string(),
        frontend_url: "http://localhost:3000".to_string(),
    }
}

#[actix_web::test]
async fn test_health_endpoint_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .route("/api/health", web::get().to(health_check))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should be valid JSON
    assert!(body_str.contains("success"));
    assert!(body_str.contains("true"));
}

#[actix_web::test]
async fn test_content_projects_endpoint_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));
    let app_config = web::Data::new(create_test_config());

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .app_data(app_config)
            .route("/api/content/{category}", web::get().to(get_content_list))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/content/project")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should be valid JSON response
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
}

#[actix_web::test]
async fn test_content_specific_project_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));
    let app_config = web::Data::new(create_test_config());

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .app_data(app_config)
            .route("/api/content/{category}/{slug}", web::get().to(get_content_item))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/content/project/project1")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // Should either succeed (if file exists) or return 404
    assert!(
        resp.status() == StatusCode::OK || resp.status() == StatusCode::NOT_FOUND
    );
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should always return valid JSON structure
    assert!(body_str.contains("success"));
    assert!(body_str.contains("message"));
}

#[actix_web::test]
async fn test_content_tags_endpoint_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));
    let app_config = web::Data::new(create_test_config());

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .app_data(app_config)
            .route("/api/content/tags", web::get().to(get_content_tags))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/content/tags")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should be valid JSON response
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
}

#[actix_web::test]
async fn test_github_projects_endpoint_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));
    let app_config = web::Data::new(create_test_config());

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .app_data(app_config)
            .route("/api/github/projects", web::get().to(get_github_projects))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/github/projects")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // GitHub API might fail due to rate limiting or network issues
    assert!(
        resp.status() == StatusCode::OK || 
        resp.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should always return valid JSON structure
    assert!(body_str.contains("success"));
    assert!(body_str.contains("message"));
}

#[actix_web::test]
async fn test_admin_refresh_github_works() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));
    let app_config = web::Data::new(create_test_config());

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .app_data(app_config)
            .route("/api/admin/refresh-github", web::post().to(refresh_github_cache))
    ).await;
    
    let req = test::TestRequest::post()
        .uri("/api/admin/refresh-github")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // Should either succeed or fail due to GitHub API issues
    assert!(
        resp.status() == StatusCode::OK || 
        resp.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should always return valid JSON structure
    assert!(body_str.contains("success"));
    assert!(body_str.contains("message"));
}

#[actix_web::test]
async fn test_admin_wrong_method_rejected() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .route("/api/admin/refresh-github", web::post().to(refresh_github_cache))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/admin/refresh-github")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // Should reject GET method (returns 404 when route doesn't exist for GET)
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_response_content_type() {
    let github_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedGithubProject>::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::<String, portfolio_backend::CachedContent>::new()));

    let app = test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
            .route("/api/health", web::get().to(health_check))
    ).await;
    
    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let content_type = resp.headers().get("content-type");
    assert!(content_type.is_some());
    
    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(content_type_str.contains("application/json"));
}