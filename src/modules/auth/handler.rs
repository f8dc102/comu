// src/modlus/auth/handler.rs

use crate::modules::auth::service::{
    delete_user, login_user, logout_user, register_user, update_user,
};
use crate::utils::db::DbPool;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

/// Register request struct
#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

/// Login request struct
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Logout request struct
#[derive(Deserialize)]
pub struct LogoutRequest {
    pub uuid: uuid::Uuid,
}

/// Update request struct
#[derive(Deserialize)]
pub struct UpdateRequest {
    pub uuid: uuid::Uuid,
    pub email: Option<String>,
    pub password: Option<String>,
}

/// Delete request struct
#[derive(Deserialize)]
pub struct DeleteRequest {
    pub uuid: uuid::Uuid,
}

/// Register handler
pub async fn register_user_handler(
    pool: web::Data<DbPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    // Call the register_user function from the service module
    match register_user(&pool, &req.email, &req.password).await {
        Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
        Err(err) => HttpResponse::BadRequest().json(json!({ "message": err })),
    }
}

/// Login handler
pub async fn login_user_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    // Call the login_user function from the service module
    match login_user(&pool, &req.email, &req.password).await {
        Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
        Err(err) => HttpResponse::Unauthorized().json(json!({ "message": err })),
    }
}

/// Logout handler
pub async fn logout_user_handler(
    pool: web::Data<DbPool>,
    req: web::Json<LogoutRequest>,
) -> impl Responder {
    // Call the logout_user function from the service module
    match logout_user(&pool, &req.uuid).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "Logged out" })),
        Err(err) => HttpResponse::Unauthorized().json(json!({ "message": err })),
    }
}

/// Update user handler
pub async fn update_user_handler(
    pool: web::Data<DbPool>,
    req: web::Json<UpdateRequest>,
) -> impl Responder {
    // Call the update_user function from the service module
    match update_user(&pool, &req.uuid, &req.email, &req.password).await {
        Ok(token) => HttpResponse::Ok().json(json!({ "token": token })),
        Err(err) => HttpResponse::Unauthorized().json(json!({ "message": err })),
    }
}

/// Delete user handler
pub async fn delete_user_handler(
    pool: web::Data<DbPool>,
    req: web::Json<DeleteRequest>,
) -> impl Responder {
    // Call the delete_user function from the service module
    match delete_user(&pool, &req.uuid).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "message": "User deleted" })),
        Err(err) => HttpResponse::Unauthorized().json(json!({ "message": err })),
    }
}

/// Verify email handler
pub async fn verify_email_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Verify email" }))
}

/// Reset password handler
pub async fn reset_password_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Reset password" }))
}

/// Refresh token handler
pub async fn refresh_token_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "Refresh token" }))
}
