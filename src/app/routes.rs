// Define all application routes in this file.
// Map URLs (e.g., `/users`, `/posts`) to route handlers.
// Example: `Router::new().route("/users", get(user_handler))`.

use axum::routing::get;
use axum::Router;
use crate::app::handlers;

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(handlers::welcome)) // Add the `welcome` route
}
