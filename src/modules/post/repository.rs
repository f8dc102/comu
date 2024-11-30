// src/modules/post/repository.rs

use crate::modules::post::model::Post;
use crate::schema::posts;

use diesel::prelude::*;
use uuid::Uuid;

/// Add a new post to the database
pub fn add_post(conn: &mut PgConnection, post: &Post) -> QueryResult<usize> {
    diesel::insert_into(posts::table).values(post).execute(conn)
}

/// Remove a post from the database
pub fn remove_post(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<usize> {
    diesel::delete(posts::table.filter(posts::uuid.eq(uuid))).execute(conn)
}

/// Modify a post in the database
pub fn modify_post(conn: &mut PgConnection, post: &Post) -> QueryResult<usize> {
    diesel::update(posts::table.filter(posts::uuid.eq(&post.uuid)))
        .set(post)
        .execute(conn)
}
