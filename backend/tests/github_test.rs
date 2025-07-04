mod common;

use actix_web::{test, http::StatusCode};
use portfolio_backend::models::{ApiResponse, GitHubProject};
use serde_json;

#[actix_web::test]
async fn test_get_github_projects_structure() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/github/projects").to_request();
    let resp = test::call_service(&app, req).await;
    
    // GitHub API might fail due to rate limiting or network issues
    // So we test both success and potential failure scenarios
    match resp.status() {
        StatusCode::OK => {
            let body = test::read_body(resp).await;
            let response: ApiResponse<Vec<GitHubProject>> = serde_json::from_slice(&body)
                .expect("Should return valid JSON response");
            
            assert!(response.success);
            assert!(response.data.is_some());
            
            let projects = response.data.unwrap();
            // Verify project structure if we have any
            for project in &projects {
                assert!(!project.owner.is_empty());
                assert!(!project.repo.is_empty());
                assert!(!project.display_name.is_empty());
                assert!(!project.url.is_empty());
                assert!(project.url.starts_with("https://github.com/"));
                
                // These fields should be present even if 0
                assert!(project.stars >= 0);
                assert!(project.forks >= 0);
            }
        },
        StatusCode::INTERNAL_SERVER_ERROR => {
            // This is acceptable if GitHub API is unavailable or rate limited
            let body = test::read_body(resp).await;
            let response: ApiResponse<Vec<GitHubProject>> = serde_json::from_slice(&body)
                .expect("Should return valid JSON error response");
            
            assert!(!response.success);
            assert!(response.data.is_none());
            assert!(!response.message.is_empty());
        },
        _ => panic!("Unexpected status code: {}", resp.status()),
    }
}

#[actix_web::test]
async fn test_github_projects_response_format() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/github/projects").to_request();
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("Response should be valid JSON");
    
    // Verify response structure exists regardless of success/failure
    assert!(json.get("success").is_some());
    assert!(json.get("message").is_some());
    
    // Check if data field exists and has correct type when present
    if let Some(data) = json.get("data") {
        if !data.is_null() {
            assert!(data.is_array(), "Data should be an array of projects");
        }
    }
}

#[actix_web::test]
async fn test_github_config_loading() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/github/projects").to_request();
    let resp = test::call_service(&app, req).await;
    
    // The endpoint should at least attempt to load the config
    // It shouldn't return a client error (which would indicate bad request)
    assert!(
        resp.status() == StatusCode::OK || resp.status() == StatusCode::INTERNAL_SERVER_ERROR,
        "Should not return client error for valid request"
    );
}

#[actix_web::test]
async fn test_github_url_format() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/github/projects").to_request();
    let resp = test::call_service(&app, req).await;
    
    if resp.status() == StatusCode::OK {
        let body = test::read_body(resp).await;
        let response: ApiResponse<Vec<GitHubProject>> = serde_json::from_slice(&body)
            .expect("Should return valid JSON response");
        
        if let Some(projects) = response.data {
            for project in &projects {
                // Verify URL format
                assert!(project.url.starts_with("https://github.com/"));
                assert!(project.url.contains(&project.owner));
                assert!(project.url.contains(&project.repo));
                
                // URL should follow pattern: https://github.com/{owner}/{repo}
                let expected_url = format!("https://github.com/{}/{}", project.owner, project.repo);
                assert_eq!(project.url, expected_url);
            }
        }
    }
}

#[actix_web::test]
async fn test_github_readme_html_format() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/github/projects").to_request();
    let resp = test::call_service(&app, req).await;
    
    if resp.status() == StatusCode::OK {
        let body = test::read_body(resp).await;
        let response: ApiResponse<Vec<GitHubProject>> = serde_json::from_slice(&body)
            .expect("Should return valid JSON response");
        
        if let Some(projects) = response.data {
            for project in &projects {
                // README should be converted to HTML or show unavailable message
                assert!(!project.readme_html.is_empty());
                
                if project.readme_html != "README not available" {
                    // If README was successfully fetched and converted,
                    // it should contain HTML tags
                    let has_html_tags = project.readme_html.contains('<') && 
                                       project.readme_html.contains('>');
                    assert!(has_html_tags, "README should be converted to HTML");
                }
            }
        }
    }
}