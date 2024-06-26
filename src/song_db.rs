use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, extract::Path};
use sqlx::Row;

use crate::types::links::Song;
use crate::config::CURRENT_SONG;
use crate::app_state::AppState;

pub async fn get_latest_song_album(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let song = Song::<String>::from(CURRENT_SONG);

    Json(song)
}

pub async fn get_song_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let q = format!("SELECT * FROM song WHERE name='{name}'");

    let r = sqlx::query(&q)
        .fetch_one(&state.db_pool)
        .await;

    match r {
        Ok(row) => {
            let song = Song {
                name: row.get("name"),
                author: row.get("author"),
                image: row.get("song_image"),
                is_album: row.get("is_album"),
                spotify_id: row.get("spotify_id"),
                youtube_id: row.get("youtube_id"),
                soundcloud_id: row.get("soundcloud_id"),
                apple_music_id: row.get("apple_music_id"),
                bandcamp_id: row.get("bandcamp_id"),
                publish_date: row.get("publish_date"),
            };
            (StatusCode::OK, Json(song)).into_response()
        },
        Err(e) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn add_song(
    State(state): State<AppState>,
    Json(song): Json<Song<String>>
) -> StatusCode {
    let s = song;

    let q = "INSERT INTO song 
        (name,author,song_image,is_album,spotify_id,youtube_id,soundcloud_id,apple_music_id,bandcamp_id,publish_date)
        VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)";

    let r = sqlx::query(q)
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
        .await;

    match r {
        Ok(_) => {
            StatusCode::CREATED
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR, 
    }
}

pub async fn delete_song_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>, 
) -> StatusCode {
    StatusCode::OK
}

pub async fn update_song_entry(
    State(state): State<AppState>,
    Path(id): Path<u64>, 
) -> StatusCode {
    StatusCode::OK
}
