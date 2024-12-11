use crate::util::*;
use leptos::server;
#[cfg(feature = "ssr")]
use sqlx::Row;

#[cfg(feature = "ssr")]
use crate::pool::pool;
use crate::types::links::Song;

use std::ops::Range;

#[server(GetLatestRelease, "/api/song", "GetJson")]
pub async fn get_latest_release() -> Result<Song> {
    let pool = pool()?;

    let q = "SELECT * FROM 
            song 
            WHERE 
            publish_date=(SELECT MAX(publish_date) FROM song)";

    let row = sqlx::query(q).fetch_one(&pool).await?;

    Ok(Song {
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
    })
}

#[server(GetSongByName, "/api/song", "GetJson")]
pub async fn get_song_by_name(name: String) -> Result<Song> {
    let pool = pool()?;

    let q = "SELECT * FROM song WHERE name=$1";

    let row = sqlx::query(q).bind(&name).fetch_one(&pool).await?;

    Ok(Song {
        name,
        author: row.get("author"),
        image: row.get("song_image"),
        is_album: row.get("is_album"),
        spotify_id: row.get("spotify_id"),
        youtube_id: row.get("youtube_id"),
        soundcloud_id: row.get("soundcloud_id"),
        apple_music_id: row.get("apple_music_id"),
        bandcamp_id: row.get("bandcamp_id"),
        publish_date: row.get("publish_date"),
    })
}

#[server(GetRangeOfSongs, "/api/song", "GetJson")]
pub async fn get_range_of_songs(range: Range<usize>) -> Result<Vec<Song>> {
    let pool = pool()?;

    let q = "SELECT * FROM song LIMIT $1 OFFSET $2";

    let songs = sqlx::query(q)
        .bind(&range.end.to_string())
        .bind(&range.start.to_string())
        .fetch_all(&pool)
        .await?;

    if songs.len() == 0 {
        err!("Failed to find any songs!")
    } else {
        Ok(songs
            .iter()
            .map(|row| Song {
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
            })
            .collect())
    }
}

#[server(AddSong, "/api/song", "Url")]
pub async fn add_song(song: Song) -> Result<()> {
    let pool = pool()?;

    let q = "SELECT * FROM song WHERE name=$1";

    let row = sqlx::query(q).bind(&song.name).fetch_one(&pool).await;

    if let Ok(_) = row {
        return err!("Song with name already exists in DB!");
    }

    let q = "INSERT INTO song 
        (name,author,song_image,is_album,spotify_id,youtube_id,soundcloud_id,apple_music_id,bandcamp_id,publish_date)
        VALUES
        ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)";

    sqlx::query(q)
        .bind(&song.name)
        .bind(&song.author)
        .bind(&song.image)
        .bind(&song.is_album)
        .bind(&song.spotify_id)
        .bind(&song.youtube_id)
        .bind(&song.soundcloud_id)
        .bind(&song.apple_music_id)
        .bind(&song.bandcamp_id)
        .bind(&song.publish_date)
        .execute(&pool)
        .await?;

    Ok(())
}

#[server(DeleteSongByName, "/api/song", "Url")]
pub async fn delete_song_by_name(name: String) -> Result<()> {
    let pool = pool()?;

    let q = "DELETE FROM song WHERE name=$1";

    sqlx::query(q).bind(&name).execute(&pool).await?;

    Ok(())
}

#[server(UpdateSongEntry, "/api/song", "Url")]
pub async fn update_song_entry(name: String, song: Song) -> Result<()> {
    if song.name != name {
        return err!("Failed to update song entry because names do not match!");
    }

    if delete_song_by_name(name).await.is_ok() {
        add_song(song).await
    } else {
        err!("Failed to update song entry because deletion of song failed!")
    }
}
