//! Run with
//!
//! ```not_rust
//! cargo run 
//! ```

mod handlers;

use handlers::*;
use axum::{
    routing::get,
    Router,
    response::IntoResponse,
    http::StatusCode,
};
use dotenvy::dotenv;
use std::env;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    if env::var("ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_string()) != "production"
    {
        dotenv().ok();
    }

    // Print all environment variables for debugging
    tracing::info!("Environment variables:");
    for (key, value) in env::vars() {
        tracing::info!("{key}={value}");
    }

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    tracing::info!("Starting server on port {}", port);

    let trace_layer = tower_http::trace::TraceLayer::new_for_http();
    let app = Router::new()
        .route("/", get(root_get_handler))
        .route("/song", get(song_handler))
        .fallback(not_found_handler)
        .layer(trace_layer);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    tracing::info!("Listening on {}", listener.local_addr().expect("Failed to get local address"));
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}

async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Route not found".to_string())
}