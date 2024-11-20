// Handle database connections and setup here.
// Example: Create a connection pool using SQLx or Diesel.

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


// use diesel::pg::PgConnection;
// use axum::{extract::Json, Extension};
// use diesel::r2d2::{self, ConnectionManager};
// use std::env;
// // use axum::extract::Json;
// use crate::models::user;
// use std::sync::Arc;
// use tokio::sync::OnceCell;
// use uuid::Uuid; // For UUID generation
// use crate::schema::users::dsl::users; // For Diesel's schema DSL
// pub type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

// // Use a OnceCell for a globally shared connection pool
// static DB_POOL: OnceCell<DbPool> = OnceCell::const_new();

// // pub async fn establish_connection() -> DbPool {
// //     DB_POOL
// //         .get_or_init(|| async {
// //             let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
// //             let manager = ConnectionManager::<PgConnection>::new(database_url);
// //             let pool = r2d2::Pool::builder()
// //                 .build(manager)
// //                 .expect("Failed to create database connection pool");
// //             Arc::new(pool)
// //         })
// //         .await
// //         .clone()
// // }

// pub async fn create_user(
//     Json(mut new_user): Json<user::NewUser>,
//     Extension(pool): Extension<DbPool>,
// ) -> Json<String> {
//     let pool = pool.clone(); // Clone the Arc to share the pool safely

//     let result = tokio::task::spawn_blocking(move || {
//         let mut conn = pool.get().expect("Failed to get DB connection");

//         // Generate a unique UUID for the user
//         new_user.user_uuid = Uuid::new_v4().to_string();

//         // Insert the new user into the database
//         diesel::insert_into(users)
//             .values(&new_user)
//             .execute(&mut *conn) // Convert PooledConnection to &mut PgConnection
//     })
//     .await
//     .expect("Task panicked");

//     match result {
//         Ok(_) => Json("User created successfully".to_string()),
//         Err(e) => Json(format!("Failed to create user: {}", e)),
//     }
// }
