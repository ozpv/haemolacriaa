use icondata::Icon;
use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;
use std::cell::Cell;

use crate::config::USERNAME;
use crate::types::images::Image;

use Platform::{AppleMusic, Bandcamp, SoundCloud, Spotify, YouTube};

// Social media info section

#[derive(PartialEq)]
pub struct SocialMediaInfo {
    pub icon: Icon,
    pub url: &'static str,
    pub active: bool,
}

// Streaming platform section

pub struct StreamingPlatform {
    pub icon: Icon,
    pub id: Platform<&'static str>,
}

pub struct StreamingInfo {
    pub platform_name: &'static str,
    pub song_url: String,
    pub platform_icon: Icon,
}

// Song info section

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Platform<T = &'static str> {
    Spotify(T),
    YouTube(T),
    SoundCloud(T),
    AppleMusic(T),
    Bandcamp(T),
}

impl<T> Platform<T> {
    /// Extracts the name of the platform in lowercase
    pub fn name(&self) -> &'static str {
        match self {
            Spotify(_) => "spotify",
            YouTube(_) => "youtube",
            SoundCloud(_) => "soundcloud",
            AppleMusic(_) => "apple music",
            Bandcamp(_) => "bandcamp",
        }
    }

    pub fn icon(&self) -> icondata::Icon {
        match self {
            Spotify(_) => icondata::SiSpotify,
            YouTube(_) => icondata::SiYoutube,
            SoundCloud(_) => icondata::SiSoundcloud,
            AppleMusic(_) => icondata::SiApple,
            Bandcamp(_) => icondata::SiBandcamp,
        }
    }

    pub fn build_url(&self, is_album: bool) -> String
    where
        T: std::fmt::Display,
    {
        match self {
            Spotify(x) => format!(
                "https://open.spotify.com/{}/{x}",
                if is_album { "album" } else { "track" }
            ),
            YouTube(x) => format!(
                "https://www.youtube.com/{}{x}",
                if is_album {
                    "playlist?list="
                } else {
                    "watch?v="
                }
            ),
            SoundCloud(x) => format!(
                "https://soundcloud.com/{USERNAME}/{}{x}",
                if is_album { "sets/" } else { "" }
            ),
            AppleMusic(x) => format!("https://music.apple.com/album/{x}"),
            Bandcamp(x) => format!(
                "https://{USERNAME}.bandcamp.com/{}/{x}",
                if is_album { "album/" } else { "track/" }
            ),
        }
    }

    pub fn unwrap(&self) -> &T {
        match self {
            Spotify(x) | YouTube(x) | SoundCloud(x) | AppleMusic(x) | Bandcamp(x) => x,
        }
    }
}

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

/*
impl<'a> From<Song<&'a str>> for Song<String> {
    fn from(s: Song<&'a str>) -> Song<String> {
        Song {
            name: s.name.to_string(),
            author: s.author.to_string(),
            image: s.image.into(),
            is_album: s.is_album,
            spotify_id: s.spotify_id.map(ToOwned::to_owned),
            youtube_id: s.youtube_id.map(ToOwned::to_owned),
            soundcloud_id: s.soundcloud_id.map(ToOwned::to_owned),
            apple_music_id: s.apple_music_id.map(ToOwned::to_owned),
            bandcamp_id: s.bandcamp_id.map(ToOwned::to_owned),
            publish_date: s.publish_date,
        }
    }
}
*/

impl<T: std::fmt::Display> Song<T> {
    pub fn get_key(&self) -> String {
        format!(
            "{}{}",
            self.name,
            if self.is_album { "album" } else { "song" },
        )
    }

    pub fn build_streaming_info(self) -> Vec<StreamingInfo> {
        let mut res = Cell::new(Vec::with_capacity(5));

        let () = self.spotify_id.map_or((), |p| {
            res.get_mut().push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.name(),
                song_url: p.build_url(self.is_album),
            });
        });

        let () = self.youtube_id.map_or((), |p| {
            res.get_mut().push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.name(),
                song_url: p.build_url(self.is_album),
            });
        });

        let () = self.soundcloud_id.map_or((), |p| {
            res.get_mut().push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.name(),
                song_url: p.build_url(self.is_album),
            });
        });

        let () = self.apple_music_id.map_or((), |p| {
            res.get_mut().push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.name(),
                song_url: p.build_url(self.is_album),
            });
        });

        let () = self.bandcamp_id.map_or((), |p| {
            res.get_mut().push(StreamingInfo {
                platform_icon: p.icon(),
                platform_name: p.name(),
                song_url: p.build_url(self.is_album),
            });
        });

        res.into_inner()
    }
}
