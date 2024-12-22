// src/modules/comment/model.rs

use crate::schema::comments;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = comments)]
pub struct Post {
    pub uuid: Uuid,
    pub content: String,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = comments)]
pub struct PostUpdate {
    pub content: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}
