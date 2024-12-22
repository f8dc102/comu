// src/modules/auth/repository.rs

use crate::modules::auth::model::{User, UserUpdate};
use crate::schema::users;

use diesel::prelude::*;
use uuid::Uuid;

/// Create a user in the database
pub fn add_user(conn: &mut PgConnection, user: &User) -> QueryResult<usize> {
    diesel::insert_into(users::table).values(user).execute(conn)
}

/// Read a user from the database
pub fn find_user_by_uuid(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<User> {
    users::table.filter(users::uuid.eq(uuid)).first(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    users::table.filter(users::email.eq(email)).first(conn)
}

/// Update a user in the database
pub fn modify_user(
    conn: &mut PgConnection,
    uuid: &Uuid,
    updated_user: &UserUpdate,
) -> QueryResult<usize> {
    use crate::schema::users::dsl::{users, uuid as user_uuid};

    diesel::update(users.filter(user_uuid.eq(uuid)))
        .set(updated_user)
        .execute(conn)
}

/// Delete a user in the database
pub fn remove_user(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<usize> {
    diesel::delete(users::table.filter(users::uuid.eq(uuid))).execute(conn)
}
