// src/modules/auth/mod.rs

pub mod handler;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod service;

use handler::{
    delete_user_handler, login_user_handler, logout_user_handler, refresh_token_handler,
    register_user_handler, reset_password_handler, verify_email_handler,
};

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register_user_handler))
            .route("/login", web::post().to(login_user_handler))
            .route("/logout", web::post().to(logout_user_handler))
            .route("/delete", web::delete().to(delete_user_handler))
            .route("/reset", web::post().to(reset_password_handler))
            .route("/verify", web::get().to(verify_email_handler))
            .route("/refresh", web::post().to(refresh_token_handler))
            .route("/reset/{token}", web::post().to(reset_password_handler))
            .route("/verify/{token}", web::get().to(verify_email_handler))
            .route("/refresh/{token}", web::post().to(refresh_token_handler)),
    );
}
