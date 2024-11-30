// src/modules/auth/repository.rs

use crate::modules::auth::model::User;
use crate::schema::users;

use diesel::prelude::*;
use uuid::Uuid;

/// Add a new user to the database
pub fn add_user(conn: &mut PgConnection, user: &User) -> QueryResult<usize> {
    diesel::insert_into(users::table).values(user).execute(conn)
}

/// Remove a user from the database
pub fn remove_user(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<usize> {
    diesel::delete(users::table.filter(users::uuid.eq(uuid))).execute(conn)
}

// @TODO: Implement update_user function

/// Find a user uuid by method such as email, username and token
/// pub fn find_user_uuid(conn: &mut PgConnection, method: String, key: String) -> QueryResult<User> {
///     // Method to find user
///     let user: User = match method.as_str() {
///         "email" => users::table.filter(users::email.eq(key)).first(conn),
///         "username" => users::table.filter(users::username.eq(key)).first(conn),
///         // "token" => users::table.filter(users::token.eq(key)).first(conn),
///         _ => panic!("Invalid method"),
///         };
/// }

/// Find a user by email
pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    users::table.filter(users::email.eq(email)).first(conn)
}
