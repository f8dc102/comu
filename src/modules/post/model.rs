// src/modules/post/model.rs

use crate::schema::posts;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub uuid: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
