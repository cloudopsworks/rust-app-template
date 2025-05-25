// tests/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    use actix_web::{test, App};
    use actix_web::http::StatusCode;
    use serde_json::Value;
    use hello_api::create_app;

    #[actix_web::test]
    async fn test_hello_integration() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, world!");
    }

    #[actix_web::test]
    async fn test_health_integration() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let json: Value = test::read_body_json(resp).await;
        assert_eq!(json["status"], "OK");
    }
}
