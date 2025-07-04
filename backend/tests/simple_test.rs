use actix_web::{test, web, App, http::StatusCode};
use portfolio_backend::*;
use std::collections::HashMap;
use std::sync::Mutex;

async fn create_app() -> impl actix_web::dev::Service<
    actix_web::dev::ServiceRequest,
    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
    Error = actix_web::Error,
> {
    let github_cache = web::Data::new(Mutex::new(HashMap::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::new()));

    test::init_service(
        App::new()
            .app_data(github_cache)
            .app_data(content_cache)
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
    ).await
}

#[actix_web::test]
async fn test_health_endpoint() {
    let app = create_app().await;
    
    let req = test::TestRequest::get()
        .uri("/api/health")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    
    // Should be valid JSON
    assert!(body_str.contains("success"));
    assert!(body_str.contains("data"));
    assert!(body_str.contains("message"));
}

#[actix_web::test]
async fn test_content_projects_endpoint() {
    let app = create_app().await;
    
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
async fn test_content_specific_project() {
    let app = create_app().await;
    
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
async fn test_content_tags_endpoint() {
    let app = create_app().await;
    
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
async fn test_github_projects_endpoint() {
    let app = create_app().await;
    
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
async fn test_admin_refresh_github() {
    let app = create_app().await;
    
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
async fn test_admin_wrong_method() {
    let app = create_app().await;
    
    let req = test::TestRequest::get()
        .uri("/api/admin/refresh-github")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    // Should reject GET method
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_nonexistent_endpoint() {
    let app = create_app().await;
    
    let req = test::TestRequest::get()
        .uri("/api/nonexistent")
        .to_request();
        
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_json_content_type() {
    let app = create_app().await;
    
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