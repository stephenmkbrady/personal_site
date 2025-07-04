mod common;

use actix_web::{test, http::StatusCode};
use portfolio_backend::models::ApiResponse;
use serde_json;

#[actix_web::test]
async fn test_health_check_returns_200() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/health");
    let resp = test::call_service(&app, req).await;
    
    common::assert_success(resp.status());
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_health_check_returns_valid_json() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/health");
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<String> = serde_json::from_slice(&body)
        .expect("Health check should return valid JSON");
    
    assert!(response.success);
    assert!(response.data.is_some());
    assert_eq!(response.message, "Success");
}

#[actix_web::test]
async fn test_health_check_content_type() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/health");
    let resp = test::call_service(&app, req).await;
    
    let content_type = resp.headers().get("content-type");
    assert!(content_type.is_some());
    
    let content_type_str = content_type.unwrap().to_str().unwrap();
    assert!(content_type_str.contains("application/json"));
}

#[actix_web::test]
async fn test_health_check_response_structure() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/health");
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Verify response structure
    assert!(json.get("success").is_some());
    assert!(json.get("data").is_some());
    assert!(json.get("message").is_some());
    
    // Verify types
    assert!(json["success"].is_boolean());
    assert!(json["data"].is_string());
    assert!(json["message"].is_string());
}