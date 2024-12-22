// src/errors/handler.rs

use actix_web::{Error, HttpResponse};
use log::error;

/// Handle errors and return appropriate response
pub async fn handle_error(err: Error) -> HttpResponse {
    // Log the error
    error!("[ERROR] {:?}", err);

    // Return an internal server error response
    HttpResponse::InternalServerError().json("Internal server error")
}
