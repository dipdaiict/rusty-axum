// src/routes/user_routes.rs

use axum::{routing::post, Router};
use sqlx::PgPool;
use crate::handler::user_handlers::create_user_handler;

pub fn create_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/register-user", post(create_user_handler))
        .with_state(pool)
}