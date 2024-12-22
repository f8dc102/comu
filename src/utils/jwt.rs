// src/utils/jwt.rs

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

/// Claims struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // General Fields
    // Use `sub` (subject) to store the user identifier (e.g. email or UUID)
    pub sub: String,
    // Use `iss` (issuer) to store the issuer of the token
    pub iss: String,
    // Use `exp` (expiration) to store the expiration time of the token
    pub exp: DateTime<Utc>,
    // Use `iat` (issued at) to store the time at which the token was issued
    pub iat: DateTime<Utc>,
    // Use `nbf` (not before) to store the time before which the token cannot be accepted
    pub nbf: Option<DateTime<Utc>>,
    // Use `aud` (audience) to store the audience of the token
    pub aud: Option<String>,
    // Use `jti` (JWT ID) to store a unique identifier for the token
    pub jti: String,

    // Custom Fields
    // Use `role` to store the role of the user
    pub role: Vec<String>,
    // Use `email` to store the email of the user
    pub email: Option<String>,
}

/// Generate a JWT token function
pub fn generate_jwt(
    secret_key: &str,
    user_id: &str,
    expiration: Duration,
    roles: Vec<String>,
    email: Option<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now: DateTime<Utc> = Utc::now();
    let claims: Claims = Claims {
        sub: user_id.to_string(),
        // @TODO: Make this configurable
        iss: "comu".to_string(),
        exp: now + expiration,
        iat: now,
        nbf: Some(now),
        // @TODO: Make this configurable
        aud: Some("doggy".to_string()),
        jti: uuid::Uuid::new_v4().to_string(),
        role: roles,
        email,
    };

    // Create a JWT token
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
}

/// Validate a JWT token function
pub fn validate_jwt(secret_key: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation: Validation = Validation::default();
    validation.set_issuer(&["comu"]);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )
    .map(|data: TokenData<Claims>| data.claims)
}
