// src/modules/auth/jwt.rs

use actix_web::HttpRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Use `sub` (subject) to store the user identifier (e.g. email or UUID)
    pub sub: String,
    // Use `exp` (expiration) to store the expiration time of the token
    pub exp: usize,
}

/// Generate a JWT token for a user
pub fn generate_jwt(user_id: &str) -> String {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now() + Duration::hours(24); // @TODO: Make this configurable

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration.timestamp() as usize,
    };

    encode(
        &jsonwebtoken::Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to generate JWT")
}

/// Validate a JWT token from an HTTP request and return the token claims
pub fn validate_jwt(req: &HttpRequest) -> Result<Claims, actix_web::Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Get the Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Bearer prefix"))?;

    // Decode and validate the JWT
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
    .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid or expired token"))
}
