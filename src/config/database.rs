// Handle database connections and setup here.
// Example: Create a connection pool using SQLx or Diesel.

// Path: src/config/database.rs

use std::env;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}
