// src/modules/post/repository.rs

use crate::modules::post::model::Post;
use crate::schema::posts;

use diesel::prelude::*;
use uuid::Uuid;

/// Add a new post to the database
pub fn add_post(conn: &mut PgConnection, post: &Post) -> QueryResult<usize> {
    diesel::insert_into(posts::table).values(post).execute(conn)
}

/// Find a post in the database
pub fn find_post_by_uuid(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<Post> {
    posts::table.filter(posts::uuid.eq(uuid)).first(conn)
}

pub fn find_posts_by_author_id(
    conn: &mut PgConnection,
    author_id: &Uuid,
) -> QueryResult<Vec<Post>> {
    posts::table
        .filter(posts::author_id.eq(author_id))
        .load(conn)
}

/// Modify a post in the database
pub fn modify_post(conn: &mut PgConnection, post: &Post) -> QueryResult<usize> {
    diesel::update(posts::table.filter(posts::uuid.eq(&post.uuid)))
        .set(post)
        .execute(conn)
}

/// Remove a post from the database
pub fn remove_post(conn: &mut PgConnection, uuid: &Uuid) -> QueryResult<usize> {
    diesel::delete(posts::table.filter(posts::uuid.eq(uuid))).execute(conn)
}
