// src/errors/mod.rs

pub mod app_error;
pub mod handler;

pub use app_error::AppError;
pub use handler::handle_error;
