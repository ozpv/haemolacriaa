use serde::{Deserialize, Serialize};

use super::images::Image;
use super::links::{Platform, StreamingInfo};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Song<T = &'static str> {
    pub name: T,
    pub author: T,
    pub image: Image<T>,
    pub is_album: bool,
    pub spotify_id: Option<Platform<T>>,
    pub youtube_id: Option<Platform<T>>,
    pub soundcloud_id: Option<Platform<T>>,
    pub apple_music_id: Option<Platform<T>>,
    pub bandcamp_id: Option<Platform<T>>,
    pub publish_date: Option<chrono::NaiveDate>,
}

impl<'a> From<Song<&'a str>> for Song<String> {
    fn from(s: Song<&'a str>) -> Song<String> {
        Song {
            name: s.name.to_string(),
            author: s.author.to_string(),
            image: s.image.into(),
            is_album: s.is_album,
            spotify_id: s.spotify_id.map(Platform::<&'a str>::cloned),
            youtube_id: s.youtube_id.map(Platform::<&'a str>::cloned),
            soundcloud_id: s.soundcloud_id.map(Platform::<&'a str>::cloned),
            apple_music_id: s.apple_music_id.map(Platform::<&'a str>::cloned),
            bandcamp_id: s.bandcamp_id.map(Platform::<&'a str>::cloned),
            publish_date: s.publish_date,
        }
    }
}

impl<T: std::fmt::Display> Song<T> {
    pub fn get_key(&self) -> String {
        format!(
            "{}{}",
            self.name,
            if self.is_album { "album" } else { "song" },
        )
    }

    pub fn build_streaming_info(self) -> Vec<StreamingInfo> {
        let mut res = Vec::with_capacity(5);

        // order these by significance
        if let Some(p) = self.spotify_id {
            res.push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.display(),
                song_url: p.build_url(self.is_album),
            });
        }

        if let Some(p) = self.youtube_id {
            res.push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.display(),
                song_url: p.build_url(self.is_album),
            });
        }

        if let Some(p) = self.soundcloud_id {
            res.push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.display(),
                song_url: p.build_url(self.is_album),
            });
        }

        if let Some(p) = self.apple_music_id {
            res.push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.display(),
                song_url: p.build_url(self.is_album),
            });
        }

        if let Some(p) = self.bandcamp_id {
            res.push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.display(),
                song_url: p.build_url(self.is_album),
            });
        }

        res
    }
}
