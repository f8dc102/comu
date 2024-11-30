// src/modules/auth/mod.rs

pub mod handler;
pub mod jwt;
pub mod middleware;
mod model;
mod repository;
mod service;

use handler::{delete, login, logout, protected, register};
use middleware::JwtMiddleware;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/delete", web::delete().to(delete))
            .route("/protected", web::get().to(protected))
            .wrap(JwtMiddleware),
    );
}
