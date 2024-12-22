// src/modules/post/service.rs

use crate::modules::post::model::Post;
use crate::modules::post::repository::{add_post, find_post, list_posts, modify_post, remove_post};
use crate::utils::db::DbPool;

use uuid::Uuid;

/// Create a new post in the database
pub async fn create_post(
    pool: &DbPool,
    title: &str,
    content: &str,
    author_id: &Uuid,
) -> Result<Post, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Build post object
    let post = Post {
        uuid: Uuid::new_v4(),
        title: title.to_string(),
        content: content.to_string(),
        author_id: *author_id,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // Create post in the database
    add_post(&mut conn, &post).map_err(|_| "Failed to create post")?;

    // Return success
    Ok(post)
}

/// Get a post from the database
pub async fn get_post(pool: &DbPool, post_id: &Uuid) -> Result<Post, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Fetch post from the database
    let post = find_post(&mut conn, post_id).map_err(|_| "Post not found")?;

    // Return success
    Ok(post)
}

/// Delete a post from the database
pub async fn delete_post(pool: &DbPool, post_id: &Uuid) -> Result<String, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Delete post from the database
    remove_post(&mut conn, post_id).map_err(|_| "Failed to delete post")?;

    // Return success
    Ok("Post deleted".to_string())
}

/// Update a post in the database
pub async fn update_post(
    pool: &DbPool,
    post_id: &Uuid,
    title: Option<String>,
    content: Option<String>,
) -> Result<Post, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Fetch post from the database
    let mut post = find_post(&mut conn, post_id).map_err(|_| "Post not found")?;

    // Update post fields
    post.title = title
        .map(|t| t.to_string())
        .unwrap_or_else(|| "".to_string());
    post.content = content
        .map(|c| c.to_string())
        .unwrap_or_else(|| "".to_string());
    post.updated_at = chrono::Utc::now().naive_utc();

    // Update post in the database
    modify_post(&mut conn, &post).map_err(|_| "Failed to update post")?;

    // Return success
    Ok(post)
}

/// List all posts from the database
pub async fn list_posts(pool: &DbPool) -> Result<Vec<Post>, String> {
    // Connect to the database
    let mut conn = pool.get().map_err(|_| "Failed to get DB connection")?;

    // Fetch posts from the database
    let posts = find_post(&mut conn).map_err(|_| "Failed to fetch posts")?;

    // Return success
    Ok(posts)
}
