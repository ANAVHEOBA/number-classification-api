// src/main.rs

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::env;

mod handlers;
mod models;
mod services;

use services::number_service::NumberService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get port from environment variable, default to 8080
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let host = "0.0.0.0";

    log::info!("Starting server at {}:{}", host, port);

    // Create shared NumberService
    let number_service = actix_web::web::Data::new(NumberService::new());

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(number_service.clone())
            .service(
                actix_web::web::resource("/api/classify-number")
                    .route(actix_web::web::get().to(handlers::classify_number)),
            )
    })
    .bind((host, port))?
    .run()
    .await
}