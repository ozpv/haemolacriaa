use axum::{http::StatusCode, response::IntoResponse, Json, extract::Path};

use crate::types::links::Song;

pub async fn get_latest_song_album() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn get_song_by_id(
    Path(id): Path<u64>,
) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn add_song(
    Json(song): Json<Song>
) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn delete_song_by_id(
    Path(id): Path<u64>, 
) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn update_song_entry(
    Path(id): Path<u64>, 
) -> impl IntoResponse {
    StatusCode::OK
}
