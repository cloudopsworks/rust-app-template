// src/handlers/health.rs
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

/// JSON response for health endpoint
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

/// Handler for /health: returns { "status": "OK" }
// #[actix_web::get("/health")]
pub async fn health() -> impl Responder {
    let res = HealthResponse { status: "OK" };
    HttpResponse::Ok().json(res)
}

// #[cfg(test)]
// mod tests {
//     use std::io::Read;
//     use super::*;
//     use actix_web::{body::to_bytes, test, http::StatusCode};
//     use actix_web::body::MessageBody;
//     use serde_json::{json, to_string};
// 
//     #[actix_web::test]
//     async fn test_health_ok() {
//         let req = test::TestRequest::default().to_http_request();
//         let resp = health().await.respond_to(&req);
//         assert_eq!(resp.status(), StatusCode::OK);
// 
//         let body = resp.into_body().try_into_bytes();
//         assert_eq!(body.as_mut()., "{ \"status\": \"OK\" }");
//     }
// }
