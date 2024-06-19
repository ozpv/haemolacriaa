use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, extract::Path};
use serde::{Serialize, Deserialize};

use crate::types::links::Song;
use crate::config::CURRENT_SONG;
use crate::app_state::AppState;

pub async fn get_latest_song_album(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let song = Song::<String>::from(CURRENT_SONG);
    Json(song)
}

pub async fn get_song_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse
//Result<impl IntoResponse, (StatusCode, sqlx::Error)>
{
    let song = Song::<String>::from(CURRENT_SONG);

    Json(song) 
}

pub async fn add_song(
    State(state): State<AppState>,
    Json(song): Json<Song<String>>
) -> impl IntoResponse 
//Result<impl IntoResponse, (StatusCode, sqlx::Error)> 
{
    let s = song;

    let q = "INSERT INTO song 
        (name,author,image,is_album,spotify_id,youtube_id,soundcloud_id,apple_music_id,bandcamp_id,publish_date)
        VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)";

    sqlx::query(q)
        .bind(&s.name)
        .bind(&s.author)
        .bind(&s.image)
        .bind(&s.is_album)
        .bind(&s.spotify_id)
        .bind(&s.youtube_id)
        .bind(&s.soundcloud_id)
        .bind(&s.apple_music_id)
        .bind(&s.bandcamp_id)
        .bind(&s.publish_date)
        .execute(&state.db_pool)
        .await
        .expect("Couldn't add song!");

    println!("added song to db!");

    StatusCode::CREATED
}

pub async fn delete_song_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>, 
) -> StatusCode {
    StatusCode::OK
}

pub async fn update_song_entry(
    State(state): State<AppState>,
    Path(id): Path<u64>, 
) -> StatusCode {
    StatusCode::OK
}
