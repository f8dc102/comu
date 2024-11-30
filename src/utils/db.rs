// src/utils/db.rs

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

// Define the database pool type
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
