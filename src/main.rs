// Entry point for the application.

use axum::{
    routing::Router,
    Extension,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use crate::app::routes::app_routes;
use crate::config::database::establish_connection;
use tracing_subscriber;

mod app;
mod config;
mod models;
mod utils;

// Constants
const SERVER_PORT: &str = "8080";
const SERVER_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    // Establish a database connection pool
    let pool = establish_connection().await;

    // Set up the application with routes and middleware
    let app = Router::new()
        .merge(app_routes())
        .layer(Extension(pool));

    // Fetch host and port from environment variables
    let (host, port) = from_env();
    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Failed to parse host and port");

    println!("🚀 Server running on {}", addr);

    // Start the server with graceful shutdown
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Fetch host and port from environment variables or use defaults
fn from_env() -> (String, u16) {
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok()) // Ensure it's a valid port number
        .unwrap_or(3000); // Default to 3000 if parsing or variable is missing
    (host, port)
}