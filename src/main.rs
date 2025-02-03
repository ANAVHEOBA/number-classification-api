use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

mod handlers;
mod models;
mod services;

use services::number_service::NumberService;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get port from environment variable for Render compatibility, default to 8080
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let host = "0.0.0.0";

    log::info!("Starting server at {}:{}", host, port);

    // Create shared number service
    let number_service = web::Data::new(NumberService::new());

    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(number_service.clone())
            .route("/api/classify-number", web::get().to(handlers::classify_number))
    })
    .bind((host, port))?
    .run()
    .await
}