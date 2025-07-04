mod common;

use actix_web::{test, http::StatusCode};
use portfolio_backend::models::{ApiResponse, ContentItem};
use serde_json;

#[actix_web::test]
async fn test_get_content_list_projects() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/project").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_success(resp.status());
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<Vec<ContentItem>> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(response.success);
    assert!(response.data.is_some());
    
    let projects = response.data.unwrap();
    assert!(!projects.is_empty(), "Should have at least one project");
    
    // Verify each project has required fields
    for project in &projects {
        assert!(!project.slug.is_empty());
        assert!(!project.metadata.title.is_empty());
        assert!(!project.metadata.description.is_empty());
        assert!(!project.html_content.is_empty());
        assert_eq!(project.category, "project");
    }
}

#[actix_web::test]
async fn test_get_content_list_blog() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/blog").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_success(resp.status());
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<Vec<ContentItem>> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(response.success);
    assert!(response.data.is_some());
    
    let blog_posts = response.data.unwrap();
    assert!(!blog_posts.is_empty(), "Should have at least one blog post");
    
    // Verify each blog post has required fields
    for post in &blog_posts {
        assert!(!post.slug.is_empty());
        assert!(!post.metadata.title.is_empty());
        assert!(!post.metadata.description.is_empty());
        assert!(!post.html_content.is_empty());
        assert_eq!(post.category, "blog");
    }
}

#[actix_web::test]
async fn test_get_content_list_invalid_category() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;
    
    // Should return success but with empty list for non-existent category
    common::assert_success(resp.status());
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<Vec<ContentItem>> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(response.success);
    assert!(response.data.is_some());
    
    let items = response.data.unwrap();
    assert!(items.is_empty(), "Non-existent category should return empty list");
}

#[actix_web::test]
async fn test_get_specific_content_item() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/project/project1").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_success(resp.status());
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<ContentItem> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(response.success);
    assert!(response.data.is_some());
    
    let project = response.data.unwrap();
    assert_eq!(project.slug, "project1");
    assert_eq!(project.category, "project");
    assert!(!project.metadata.title.is_empty());
    assert!(!project.html_content.is_empty());
}

#[actix_web::test]
async fn test_get_nonexistent_content_item() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/project/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_client_error(resp.status());
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<ContentItem> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(!response.success);
    assert!(response.data.is_none());
    assert!(response.message.contains("not found"));
}

#[actix_web::test]
async fn test_get_content_tags() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/tags").to_request();
    let resp = test::call_service(&app, req).await;
    
    common::assert_success(resp.status());
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<Vec<String>> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    assert!(response.success);
    assert!(response.data.is_some());
    
    let tags = response.data.unwrap();
    assert!(!tags.is_empty(), "Should have at least one tag");
    
    // Verify tags are unique and sorted
    let mut prev_tag = "";
    for tag in &tags {
        assert!(!tag.is_empty());
        assert!(tag > prev_tag, "Tags should be sorted alphabetically");
        prev_tag = tag;
    }
}

#[actix_web::test]
async fn test_content_list_sorted_by_date() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/blog").to_request();
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<Vec<ContentItem>> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    let blog_posts = response.data.unwrap();
    
    if blog_posts.len() > 1 {
        // Verify posts are sorted by date (newest first)
        for i in 1..blog_posts.len() {
            let current_date = &blog_posts[i].metadata.date;
            let previous_date = &blog_posts[i-1].metadata.date;
            assert!(
                current_date <= previous_date,
                "Blog posts should be sorted by date (newest first)"
            );
        }
    }
}

#[actix_web::test]
async fn test_content_html_conversion() {
    let app = common::create_test_app().await;
    
    let req = common::create_request("GET", "/api/content/project/project1").to_request();
    let resp = test::call_service(&app, req).await;
    
    let body = test::read_body(resp).await;
    let response: ApiResponse<ContentItem> = serde_json::from_slice(&body)
        .expect("Should return valid JSON response");
    
    let project = response.data.unwrap();
    
    // Verify markdown was converted to HTML
    assert!(project.html_content.contains("<h1>") || project.html_content.contains("<h2>"));
    assert!(project.html_content.contains("<p>"));
    
    // Should not contain raw markdown syntax
    assert!(!project.html_content.contains("##"));
    assert!(!project.html_content.contains("**"));
}