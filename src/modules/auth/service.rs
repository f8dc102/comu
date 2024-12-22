// src/modules/auth/service.rs

use crate::modules::auth::model::{User, UserUpdate};
use crate::modules::auth::repository::{
    add_user, find_user_by_email, find_user_by_uuid, modify_user, remove_user,
};
use crate::utils::db::DbPool;
use crate::utils::jwt::generate_jwt;

use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

/// Register a new user in the database
pub async fn register_user(pool: &DbPool, email: &str, password: &str) -> Result<String, String> {
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
        password_hash,
        created_at: chrono::Local::now().naive_utc().into(),
        updated_at: chrono::Local::now().naive_utc().into(),
        deleted_at: None,
    };

    // Create user in the database
    add_user(&mut conn, &user).map_err(|_| "Failed to create user")?;

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

/// Logout a user
pub async fn logout_user(pool: &DbPool, user_id: &Uuid) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // @TODO: Implement logout_user function
    let (_, _) = (user_id, &mut conn);

    // Return success
    Ok("Logged out".to_string())
}

/// Update a user
pub async fn update_user(
    pool: &DbPool,
    user_id: &Uuid,
    email: &Option<String>,
    password: &Option<String>,
) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Fetch the current user
    let current_user = find_user_by_uuid(&mut conn, user_id).map_err(|_| "User not found")?;

    // Check if the email already exists
    if let Some(new_email) = email {
        if let Ok(existing_user) = find_user_by_email(&mut conn, new_email) {
            if existing_user.uuid != *user_id {
                return Err("Email is already in use".to_string());
            }
        }
    }

    // Prepare updated user fields
    let updated_user = UserUpdate {
        email: email.clone().or(Some(current_user.email)),
        password_hash: if let Some(pass) = password {
            Some(hash(pass, DEFAULT_COST).map_err(|_| "Password hashing failed")?)
        } else {
            Some(current_user.password_hash.clone())
        },
        updated_at: chrono::Local::now().naive_utc(),
    };

    // Update user in the database
    modify_user(&mut conn, user_id, &updated_user).map_err(|_| "Failed to update user")?;

    // Return success
    Ok("User updated".to_string())
}

/// Delete a user
pub async fn delete_user(pool: &DbPool, user_id: &Uuid) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Delete user from the database
    remove_user(&mut conn, user_id).map_err(|_| "Failed to delete user")?;

    // Return success
    Ok("User deleted".to_string())
}
