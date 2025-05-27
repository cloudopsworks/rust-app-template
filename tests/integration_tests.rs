// tests/integration_tests.rs
use actix_web::{test, App};
use hello_api::routes;

#[actix_web::test]
async fn test_hello_endpoint() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(routes::init_routes)
    ).await;

    // Act
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello, world!");
}

#[actix_web::test]
async fn test_health_endpoint() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(routes::init_routes)
    ).await;

    // Act
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_not_found() {
    // Arrange
    let app = test::init_service(
        App::new()
            .configure(routes::init_routes)
    ).await;

    // Act
    let req = test::TestRequest::get().uri("/non-existent").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert
    assert_eq!(resp.status().as_u16(), 404);
}