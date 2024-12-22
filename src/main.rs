// src/main.rs

mod errors;
mod modules;
mod schema;
mod utils;

use errors::AppError;
use modules::auth;
use modules::post;
use utils::db::init_pool;

use actix_web::{web, App, HttpServer};

/// Main function to start the server
#[actix_web::main]
async fn main() -> std::io::Result<(), AppError> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Create database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url).wrap_err("Failed to create pool")?;

    // Get the listen address and port
    // Default to localhost:8080
    let host = std::env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:8080".to_string());

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(auth::init_routes)
    })
    .bind(host)?
    .run()
    .await?;

    Ok(())
}
