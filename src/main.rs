// Entry point for the application.

use axum::{
    routing::Router,
    Extension,
};
use std::net::SocketAddr;
use dotenv::dotenv;
use crate::config::database::establish_connection;
use tracing_subscriber;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};
use tracing::Level;
use tracing::info;
use crate::routes::user_routes;
use crate::routes::entry_routes;

mod app;
mod config;
mod models;
mod utils;
mod routes;
mod handler;
mod schema;

// Constants
#[allow(dead_code)]
const SERVER_PORT: &str = "8080";
#[allow(dead_code)]
const SERVER_HOST: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Initialize tracing for better logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // Log before attempting to establish a database connection
    info!("Attempting to establish a database connection...");

    // Establish a database connection pool
    let pool = establish_connection().await;
    info!("✅ Database connection pool established successfully.");

    // Create the router
    let app = Router::new()
    .merge(entry_routes::app_routes()) // Merge general entry routes
    .merge(user_routes::create_routes(pool.clone())) // Pass the pool to user routes
    .layer(Extension(pool)) // Share the pool with all routes via Extension
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    // Fetch host and port from environment variables
    let (host, port) = from_env();
    let addr = format!("{}:{}", host, port)
        .parse::<SocketAddr>()
        .expect("Failed to parse host and port");

    // Dynamically fetch local and network IPs
    let network_ip = get_network_ip().unwrap_or_else(|| "Unavailable".to_string());
    info!("🚀 Server running on:");
    info!(" - Local: http://127.0.0.1:{}", port);
    info!(" - Network: http://{}:{}", network_ip, port);

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

// Geth the IP:
/// Get the network IP of the machine (IPv4 only)
fn get_network_ip() -> Option<String> {
    use get_if_addrs::get_if_addrs;

    get_if_addrs().ok()?.into_iter()
        .filter(|iface| !iface.is_loopback() && iface.ip().is_ipv4())
        .map(|iface| iface.ip().to_string())
        .next()
}