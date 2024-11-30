// src/modules/auth/service.rs

use crate::modules::auth::jwt::generate_jwt;
use crate::modules::auth::model::User;
use crate::modules::auth::repository::{create_user, find_user_by_email};
use crate::utils::db::DbPool;

use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

/// Register a new user in the database
pub async fn register_user(
    pool: &DbPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Check if the email is already registered
    let email_check = find_user_by_email(&mut conn, email);

    // If the email is already registered, return an error
    if email_check.is_ok() {
        return Err("Email already exists".to_string());
    }

    // Password hashing
    let password_hash = hash(password, DEFAULT_COST).map_err(|_| "Password hashing failed")?;

    // Build user object
    let user = User {
        uuid: Uuid::new_v4(),
        email: email.to_string(),
        username: username.to_string(),
        password_hash,
        created_at: None,
        updated_at: None,
        deleted_at: None,
    };

    // Create user in the database
    create_user(&mut conn, &user).map_err(|_| "Failed to create user")?;

    // Generate JWT token
    let token = generate_jwt(&user.uuid.to_string());

    // Return success
    Ok(token)
}

/// Login a user
pub async fn login_user(pool: &DbPool, email: &str, password: &str) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Search for user by email
    let user = find_user_by_email(&mut conn, email).map_err(|_| "Invalid email or password")?;

    // Check if the password is correct
    if !verify(password, &user.password_hash).map_err(|_| "Invalid email or password")? {
        return Err("Invalid email or password".to_string());
    }

    // Generate JWT token
    let token = generate_jwt(&user.uuid.to_string());

    // Return success
    Ok(token)
}
