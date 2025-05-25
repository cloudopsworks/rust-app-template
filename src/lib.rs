// src/lib.rs
pub mod handlers;
pub mod routes;

use actix_web::{App, middleware::Logger, middleware::DefaultHeaders};

/// Create Actix Web App with all routes configured and logging middleware
pub fn create_app() -> App<impl actix_web::dev::ServiceFactory> {
    App::new()
        .wrap(Logger::default())
        .wrap(DefaultHeaders::new().add(("X-Version", env!("CARGO_PKG_VERSION"))))
        .configure(routes::init_routes)
}
