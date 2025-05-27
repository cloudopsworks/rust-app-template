// src/routes.rs
use actix_web::web;
use crate::handlers::{hello, health};

/// Initialize all application routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/")
                .route("", web::get().to(hello::hello)))
        .service(web::scope("/health")
                .route("", web::get().to(health::health))
        );
}
