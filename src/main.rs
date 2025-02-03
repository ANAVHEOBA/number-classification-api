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

    log::info!("Starting server at http://localhost:8080");

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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}