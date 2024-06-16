use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, extract::Path};
use serde::{Serialize, Deserialize};
use std::sync::Arc;

use crate::types::links::Song;
use crate::config::CURRENT_SONG;
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct SongResponse {
    pub id: u64,
    pub song: Song,
}

pub async fn get_latest_song_album(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let song = SongResponse {
        id: 0,
        song: CURRENT_SONG.to_song(),
    };
    Json(song)
}

pub async fn get_song_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let song = SongResponse {
        id: 0,
        song: CURRENT_SONG.to_song(),
    };

    Ok(Json(song)) 
}

pub async fn add_song(
    State(state): State<Arc<AppState>>,
    Json(song): Json<Song>
) -> StatusCode {
    StatusCode::OK
}

pub async fn delete_song_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>, 
) -> StatusCode {
    StatusCode::OK
}

pub async fn update_song_entry(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>, 
) -> StatusCode {
    StatusCode::OK
}
