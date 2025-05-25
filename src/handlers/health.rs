// src/handlers/health.rs
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

/// JSON response for health endpoint
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Handler for /health: returns { "status": "OK" }
pub async fn health() -> impl Responder {
    let res = HealthResponse { status: "OK" };
    HttpResponse::Ok().json(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::to_bytes, test, http::StatusCode};
    use serde_json::json;

    #[actix_web::test]
    async fn test_health_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = health().await.respond_to(&req);
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        let expected = json!({ "status": "OK" }).to_string();
        assert_eq!(body, expected);
    }
}
