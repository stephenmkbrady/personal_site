mod common;

use actix_web::{test, http::StatusCode};
use portfolio_backend::models::{ApiResponse, ContentItem, GitHubProject};
use serde_json;

#[actix_web::test]
async fn test_full_api_workflow() {
    let app = common::create_test_app().await;
    
    // 1. Test health check first
    let health_req = common::create_request("GET", "/api/health").to_request();
    let health_resp = test::call_service(&app, health_req).await;
    assert_eq!(health_resp.status(), StatusCode::OK);
    
    // 2. Test content endpoints
    let projects_req = common::create_request("GET", "/api/content/project").to_request();
    let projects_resp = test::call_service(&app, projects_req).await;
    assert_eq!(projects_resp.status(), StatusCode::OK);
    
    let blog_req = common::create_request("GET", "/api/content/blog").to_request();
    let blog_resp = test::call_service(&app, blog_req).await;
    assert_eq!(blog_resp.status(), StatusCode::OK);
    
    // 3. Test tags endpoint
    let tags_req = common::create_request("GET", "/api/content/tags").to_request();
    let tags_resp = test::call_service(&app, tags_req).await;
    assert_eq!(tags_resp.status(), StatusCode::OK);
    
    // 4. Test GitHub endpoint (may fail due to network/API limits)
    let github_req = common::create_request("GET", "/api/github/projects").to_request();
    let github_resp = test::call_service(&app, github_req).await;
    assert!(
        github_resp.status() == StatusCode::OK || 
        github_resp.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
    
    // 5. Test admin endpoint
    let admin_req = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let admin_resp = test::call_service(&app, admin_req).await;
    assert!(
        admin_resp.status() == StatusCode::OK || 
        admin_resp.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[actix_web::test]
async fn test_content_consistency() {
    let app = common::create_test_app().await;
    
    // Get list of projects
    let list_req = common::create_request("GET", "/api/content/project").to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    
    let list_body = test::read_body(list_resp).await;
    let list_response: ApiResponse<Vec<ContentItem>> = serde_json::from_slice(&list_body).unwrap();
    let projects = list_response.data.unwrap();
    
    if !projects.is_empty() {
        let first_project = &projects[0];
        
        // Get the same project by slug
        let item_req = common::create_request(
            "GET", 
            &format!("/api/content/project/{}", first_project.slug)
        ).to_request();
        let item_resp = test::call_service(&app, item_req).await;
        assert_eq!(item_resp.status(), StatusCode::OK);
        
        let item_body = test::read_body(item_resp).await;
        let item_response: ApiResponse<ContentItem> = serde_json::from_slice(&item_body).unwrap();
        let individual_project = item_response.data.unwrap();
        
        // Verify consistency
        assert_eq!(first_project.slug, individual_project.slug);
        assert_eq!(first_project.metadata.title, individual_project.metadata.title);
        assert_eq!(first_project.html_content, individual_project.html_content);
    }
}

#[actix_web::test]
async fn test_error_responses_are_consistent() {
    let app = common::create_test_app().await;
    
    // Test various error scenarios
    let error_endpoints = vec![
        "/api/content/project/nonexistent",
        "/api/content/invalid-category/test",
    ];
    
    for endpoint in error_endpoints {
        let req = common::create_request("GET", endpoint).to_request();
        let resp = test::call_service(&app, req).await;
        
        if resp.status() == StatusCode::NOT_FOUND {
            let body = test::read_body(resp).await;
            let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
            
            // Verify error response structure
            assert!(json.get("success").is_some());
            assert!(json.get("message").is_some());
            assert_eq!(json["success"], false);
            assert!(json["message"].is_string());
        }
    }
}

#[actix_web::test]
async fn test_cors_headers() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/health").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Check for CORS headers (these are set in main.rs)
    let cors_origin = resp.headers().get("access-control-allow-origin");
    let cors_methods = resp.headers().get("access-control-allow-methods");
    let cors_headers = resp.headers().get("access-control-allow-headers");
    
    assert!(cors_origin.is_some());
    assert!(cors_methods.is_some());
    assert!(cors_headers.is_some());
}

#[actix_web::test]
async fn test_json_content_type() {
    let app = common::create_test_app().await;
    
    let endpoints = vec![
        "/api/health",
        "/api/content/project",
        "/api/content/tags",
        "/api/github/projects",
    ];
    
    for endpoint in endpoints {
        let req = common::create_request("GET", endpoint).to_request();
        let resp = test::call_service(&app, req).await;
        
        if resp.status() == StatusCode::OK {
            let content_type = resp.headers().get("content-type");
            assert!(content_type.is_some());
            
            let content_type_str = content_type.unwrap().to_str().unwrap();
            assert!(content_type_str.contains("application/json"));
        }
    }
}

#[actix_web::test]
async fn test_cache_behavior() {
    let app = common::create_test_app().await;
    
    // Make two identical requests to test caching
    let req1 = common::create_request("GET", "/api/content/project").to_request();
    let resp1 = test::call_service(&app, req1).await;
    
    let req2 = common::create_request("GET", "/api/content/project").to_request();
    let resp2 = test::call_service(&app, req2).await;
    
    assert_eq!(resp1.status(), resp2.status());
    
    if resp1.status() == StatusCode::OK {
        let body1 = test::read_body(resp1).await;
        let body2 = test::read_body(resp2).await;
        
        // Responses should be identical (content is cached)
        assert_eq!(body1, body2);
    }
}