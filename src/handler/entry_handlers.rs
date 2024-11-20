// Implement route handler functions here.
// These functions process incoming requests and return responses.
// Example: Handle logic for creating a user, fetching posts, etc.
// Path: //rusty-axum/src/handler/entry_hanlers.rs

use axum::response::Html;

/// Welcome handler
pub async fn welcome() -> Html<&'static str> {
    Html("<h1>Welcome to the Rusty-Axum Learning Project!</h1>")
}
