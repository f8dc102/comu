// src/modules/post/mod.rs

pub mod handler;
mod model;
mod repository;
mod service;

use handler::{create, delete, get, update};

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("/create", web::post().to(create))
            .route("/delete", web::post().to(delete))
            .route("/update", web::post().to(update))
            .route("/get/{id}", web::get().to(get)),
    );
}
