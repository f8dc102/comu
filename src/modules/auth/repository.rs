// src/modules/auth/repository.rs

use crate::modules::auth::model::User;
use crate::schema::users;

use diesel::prelude::*;

pub fn create_user(conn: &mut PgConnection, user: &User) -> QueryResult<usize> {
    diesel::insert_into(users::table).values(user).execute(conn)
}

pub fn find_user_by_email(conn: &mut PgConnection, email: &str) -> QueryResult<User> {
    users::table.filter(users::email.eq(email)).first(conn)
}
