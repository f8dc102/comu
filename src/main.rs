// src/main.rs

// Load the modules
mod modules;
mod schema;
mod utils;

// Import the modules
use modules::auth::handler::{login, protected, register};
use modules::auth::middleware::JwtMiddleware;
use utils::db::init_pool;

// Import the required crates
use actix_web::{web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();
    // Initialize logger
    env_logger::init();

    // Log the server start
    info!("Starting the server...");

    // Create database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/protected", web::get().to(protected))
            .wrap(JwtMiddleware)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
