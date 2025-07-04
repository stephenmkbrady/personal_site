mod common;

use actix_web::{test, http::StatusCode};
use portfolio_backend::models::ApiResponse;
use serde_json;

#[actix_web::test]
async fn test_refresh_github_cache_method() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should either succeed or fail due to GitHub API issues
    // but should accept the POST method
    assert!(
        resp.status() == StatusCode::OK || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Should accept POST method, got: {}", resp.status()
    );
}

#[actix_web::test]
async fn test_refresh_github_cache_wrong_method() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/admin/refresh-github").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should reject GET method
    common::assert_client_error(resp.status());
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_refresh_github_cache_response_format() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Verify response structure
    assert!(json.get("success").is_some());
    assert!(json.get("message").is_some());
    
    // Verify types
    assert!(json["success"].is_boolean());
    assert!(json["message"].is_string());
}

#[actix_web::test]
async fn test_refresh_github_cache_success_message() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp = test::call_service(&app, req).await;
    
    if resp.status() == StatusCode::OK {
        let body = test::read_body(resp).await;
        let response: ApiResponse<String> = serde_json::from_slice(&body)
            .expect("Should return valid JSON response");
        
        assert!(response.success);
        assert!(response.data.is_some());
        
        let message = response.data.unwrap();
        assert!(message.contains("Refreshed"));
        assert!(message.contains("projects"));
    }
}

#[actix_web::test]
async fn test_refresh_github_cache_idempotent() {
    let app = common::create_test_app().await;
    
    // First request
    let req1 = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp1 = test::call_service(&app, req1).await;
    
    // Second request immediately after
    let req2 = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp2 = test::call_service(&app, req2).await;
    
    // Both should have the same response pattern
    assert_eq!(resp1.status(), resp2.status());
    
    if resp1.status() == StatusCode::OK {
        let body1 = test::read_body(resp1).await;
        let body2 = test::read_body(resp2).await;
        
        let response1: ApiResponse<String> = serde_json::from_slice(&body1).unwrap();
        let response2: ApiResponse<String> = serde_json::from_slice(&body2).unwrap();
        
        assert_eq!(response1.success, response2.success);
        // The refresh count should be the same since we're hitting the same repos
        assert!(response1.data.is_some());
        assert!(response2.data.is_some());
    }
}

#[actix_web::test]
async fn test_admin_endpoint_not_cached() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("POST", "/api/admin/refresh-github").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Admin endpoints should not include cache headers
    let cache_control = resp.headers().get("cache-control");
    if let Some(cache_header) = cache_control {
        let cache_value = cache_header.to_str().unwrap();
        assert!(
            cache_value.contains("no-cache") || cache_value.contains("no-store"),
            "Admin endpoints should not be cached"
        );
    }
}

#[actix_web::test]
async fn test_nonexistent_admin_endpoint() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("POST", "/api/admin/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_client_error(resp.status());
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}