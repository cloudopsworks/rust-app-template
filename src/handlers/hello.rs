// src/handlers/hello.rs
use actix_web::{HttpResponse, Responder, web};

/// Handler for the root endpoint: returns "Hello, world!"
// #[actix_web::get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{body::to_bytes, test, http::StatusCode};
// 
//     #[actix_web::test]
//     async fn test_hello_ok() {
//         let req = test::TestRequest::default().to_http_request();
//         let resp = hello().await.respond_to(&req);
//         assert_eq!(resp.status(), StatusCode::OK);
// 
//         let body = to_bytes(resp.into_body()).await;
//         assert_eq!(body.unwrap(), web::Bytes::from_static(b"Hello, world!"));
//     }
// }
