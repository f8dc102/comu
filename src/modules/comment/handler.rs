// src/modules/post/handler.rs

use crate::modules::post::service::{create_post, delete_post, get_post, update_post};
use crate::utils::db::DbPool;

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

/// Create post request struct
#[derive(Debug, Deserialize)]
pub struct CreateComment {
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
}

/// Update post request struct
#[derive(Debug, Deserialize)]
pub struct UpdateComment {
    pub uuid: Uuid,
    pub content: Option<String>,
}

/// Get post request struct
#[derive(Debug, Deserialize)]
pub struct GetPost {
    pub uuid: Uuid,
}

/// List posts request struct
#[derive(Debug, Deserialize)]
pub struct ListPosts {
    pub limit: Option<i64>,
}

/// Create post handler
pub async fn create_post_handler(
    pool: web::Data<DbPool>,
    data: web::Json<CreatePost>,
) -> impl Responder {
    // Call the create_post function from the service module
    match create_post(&pool, &data.title, &data.content, &data.author_id).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(err) => HttpResponse::BadRequest().json(json!({ "message": err })),
    }
}

/// Helper: Convert Markdown to HTML
fn render_markdown_to_html(markdown: &str) -> String {
    use pulldown_cmark::{html, Parser};

    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Get post handler
pub async fn get_post_handler(
    pool: web::Data<DbPool>,
    post_id: web::Path<Uuid>,
    query: web::Query<Option<String>>,
) -> impl Responder {
    // Call the get_post function from the service module
    match get_post(&pool, &post_id).await {
        Ok(post) => {
            if query.into_inner().unwrap_or_default() == "html" {
                // Render as HTML
                let html = render_markdown_to_html(&post.content);

                HttpResponse::Ok().body(html)
            } else {
                // Return raw Markdown
                HttpResponse::Ok().json(post)
            }
        }
        Err(err) => HttpResponse::NotFound().json(json!({ "message": err })),
    }
}

/// Update post handler
pub async fn update_post_handler(
    pool: web::Data<DbPool>,
    data: web::Json<UpdatePost>,
) -> impl Responder {
    // Validate the request
    if data.title.is_none() || data.content.is_none() {
        return HttpResponse::BadRequest()
            .json(json!({ "message": "Title or content is required" }));
    }

    // Validate the UUID
    if data.uuid.is_nil() {
        return HttpResponse::BadRequest().json(json!({ "message": "Invalid UUID" }));
    }

    // Wrap the data into a struct
    let data = UpdatePost {
        title: data.title.clone(),
        content: data.content.clone(),
        uuid: data.uuid,
    };

    // Call the update_post function from the service module
    match update_post(&pool, &data.uuid, data.title, data.content).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().json(json!({ "message": err })),
    }
}

/// Delete post handler
pub async fn delete_post_handler(
    pool: web::Data<DbPool>,
    post_id: web::Path<Uuid>,
) -> impl Responder {
    // Call the delete_post function from the service module
    match delete_post(&pool, &post_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => HttpResponse::NotFound().json(json!({ "message": err })),
    }
}
