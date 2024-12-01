// src/modules/post/mod.rs

pub mod handler;
mod model;
mod repository;
mod service;

use handler::{create_post_handler, delete_post_handler, get_post_handler, update_post_handler};

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .route("/create", web::post().to(create_post_handler))
            .route("/get/{id}", web::get().to(get_post_handler))
            .route("/update", web::post().to(update_post_handler))
            .route("/delete", web::post().to(delete_post_handler)),
    );
}
