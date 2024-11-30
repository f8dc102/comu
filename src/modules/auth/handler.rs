// src/modlus/auth/handler.rs

use crate::modules::auth::service::{login_user, register_user};
use crate::utils::db::DbPool;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

/// Register request struct
#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

/// Login request struct
#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Register handler
pub async fn register(pool: web::Data<DbPool>, req: web::Json<RegisterRequest>) -> impl Responder {
    // Call the register_user function from the service module
    match register_user(&pool, &req.email, &req.username, &req.password).await {
        Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
        Err(err) => HttpResponse::BadRequest().json(json!({ "message": err })),
    }
}

/// Login handler
pub async fn login(pool: web::Data<DbPool>, req: web::Json<LoginRequest>) -> impl Responder {
    // Call the login_user function from the service module
    match login_user(&pool, &req.email, &req.password).await {
        Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
        Err(err) => HttpResponse::Unauthorized().json(json!({ "message": err })),
    }
}

/// Protected handler for testing JWT Middleware
/// @TODO: Remove this handler in production
pub async fn protected() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "You are authorized!" }))
}
