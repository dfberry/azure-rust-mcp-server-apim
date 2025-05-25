use axum::{
    response::{Html, IntoResponse, Json},
    http::StatusCode,
};
use song::{Song, SongGenerator};

pub async fn root_get_handler() -> Html<String> {
    let html_content = format!("<h1>Rust server</h1> <p>Hello</p>");
    Html(html_content)
}

pub async fn song_handler() -> Json<Song> {
    // Use tokio::task::spawn_blocking for CPU-intensive sync operations
    let song = tokio::task::spawn_blocking(|| {
        let song_generator = SongGenerator::new();
        song_generator.get_random_song()
    })
    .await
    .unwrap_or_else(|e| {
        tracing::error!("Failed to generate song: {}", e);
        Song::default()
    });

    Json(song)
}

pub async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Route not found")
}
