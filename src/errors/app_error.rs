// src/errors/app_error.rs

use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

/// Struct to represent an error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
    // Optional field to provide more details about the error
    details: Option<String>,
}

/// Generalized error types for the application
#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    EnvVarError(String),
    Unauthorized,
    Forbidden,
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
}

/// Implement the Display trait for AppError
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::EnvVarError(msg) => write!(f, "Environment variable error: {}", msg),
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden => write!(f, "Forbidden"),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

/// Implement the ResponseError trait for AppError
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        // 에러를 직렬화된 JSON으로 반환
        let (status_code, message, details) = match self {
            AppError::DatabaseError(msg) => (
                500,
                "Database connection error".to_string(),
                Some(msg.clone()),
            ),
            AppError::EnvVarError(msg) => (
                500,
                "Environment variable error".to_string(),
                Some(msg.clone()),
            ),
            AppError::Unauthorized => (401, "Unauthorized".to_string(), None),
            AppError::Forbidden => (403, "Forbidden".to_string(), None),
            AppError::BadRequest(msg) => (400, "Bad request".to_string(), Some(msg.clone())),
            AppError::NotFound(resource) => (404, format!("{} not found", resource), None),
            AppError::InternalServerError(msg) => {
                (500, "Internal server error".to_string(), Some(msg.clone()))
            }
        };

        let response_body = ErrorResponse {
            code: status_code,
            message,
            details,
        };

        HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap())
            .json(response_body)
    }
}
