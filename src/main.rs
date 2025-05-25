// src/main.rs
use std::env;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use log::info;
use hello_api::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger from environment (default level: info)
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    info!("Starting server at http://{}:{}", host, port);

    HttpServer::new(|| App::new()
                        .wrap(Logger::default())
                        .configure(routes::init_routes))
        .bind((host, port))?
        .run()
        .await
}
