use actix_web::{test, App, web, dev::Service};
use portfolio_backend::*;
use std::collections::HashMap;
use std::sync::Mutex;

/// Create a test application instance with all routes configured
pub async fn create_test_app() -> impl Service<
    actix_web::dev::ServiceRequest,
    Response = actix_web::dev::ServiceResponse<actix_web::body::BoxBody>,
    Error = actix_web::Error,
> {
    let github_cache = web::Data::new(Mutex::new(HashMap::new()));
    let content_cache = web::Data::new(Mutex::new(HashMap::new()));

    test::init_service(
        App::new()
            .app_data(github_cache.clone())
            .app_data(content_cache.clone())
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

/// Helper to create a test request
pub fn create_request(method: &str, uri: &str) -> actix_web::dev::ServiceRequest {
    match method.to_uppercase().as_str() {
        "GET" => test::TestRequest::get().uri(uri).to_srv_request(),
        "POST" => test::TestRequest::post().uri(uri).to_srv_request(),
        "PUT" => test::TestRequest::put().uri(uri).to_srv_request(),
        "DELETE" => test::TestRequest::delete().uri(uri).to_srv_request(),
        _ => test::TestRequest::get().uri(uri).to_srv_request(),
    }
}

/// Assert that a response is successful (2xx status code)
pub fn assert_success(status: actix_web::http::StatusCode) {
    assert!(status.is_success(), "Expected success status, got: {}", status);
}

/// Assert that a response is a client error (4xx status code)
pub fn assert_client_error(status: actix_web::http::StatusCode) {
    assert!(status.is_client_error(), "Expected client error status, got: {}", status);
}

/// Assert that a response is a server error (5xx status code)
pub fn assert_server_error(status: actix_web::http::StatusCode) {
    assert!(status.is_server_error(), "Expected server error status, got: {}", status);
}