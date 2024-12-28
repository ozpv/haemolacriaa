use icondata::Icon;
use serde::{Deserialize, Serialize};
use std::borrow::ToOwned;

use crate::config::{
    PlatformId,
    PlatformId::{AppleMusic, Bandcamp, SoundCloud, Spotify, YouTube},
    STREAMING_PLATFORMS,
};
use crate::types::images::Image;

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
    pub id: PlatformId<&'static str>,
}

impl StreamingPlatform {
    fn create_url<'a, T>(
        &self,
        id: &T,
        is_album: bool,
        main: Option<&'a str>,
        alt: Option<&'a str>,
    ) -> String
    where
        T: std::fmt::Display,
    {
        format!(
            "{}{}{}",
            &self.id.unwrap(),
            if is_album {
                main.unwrap_or("")
            } else {
                alt.unwrap_or("")
            },
            &id
        )
    }
}

pub struct StreamingInfo {
    pub song_id: Option<(&'static str, String)>,
    pub platform_icon: Icon,
}

// Song info section

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Song<T = &'static str> {
    pub name: T,
    pub author: T,
    pub image: Image<T>,
    pub is_album: bool,
    pub spotify_id: Option<T>,
    pub youtube_id: Option<T>,
    pub soundcloud_id: Option<T>,
    pub apple_music_id: Option<T>,
    pub bandcamp_id: Option<T>,
    pub publish_date: Option<chrono::NaiveDate>,
}

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

impl<T: std::fmt::Display> Song<T> {
    pub fn get_key(&self) -> String {
        format!(
            "{}{}",
            self.name,
            if self.is_album { "album" } else { "song" },
        )
    }

    pub fn build_streaming_info(self) -> Vec<StreamingInfo> {
        STREAMING_PLATFORMS
            .iter()
            .map(|platform| StreamingInfo {
                platform_icon: platform.icon,
                song_id: match platform.id {
                    Spotify(_) => self.spotify_id.as_ref().map(|id| {
                        (
                            platform.id.name(),
                            platform.create_url(id, self.is_album, Some("album/"), Some("track/")),
                        )
                    }),
                    YouTube(_) => self.youtube_id.as_ref().map(|id| {
                        (
                            platform.id.name(),
                            platform.create_url(
                                id,
                                self.is_album,
                                Some("playlist?list="),
                                Some("watch?v="),
                            ),
                        )
                    }),
                    SoundCloud(_) => self.soundcloud_id.as_ref().map(|id| {
                        (
                            platform.id.name(),
                            platform.create_url(id, self.is_album, Some("sets/"), None),
                        )
                    }),
                    AppleMusic(_) => self.apple_music_id.as_ref().map(|id| {
                        (
                            platform.id.name(),
                            platform.create_url(id, false, None, None),
                        )
                    }),
                    Bandcamp(_) => self.bandcamp_id.as_ref().map(|id| {
                        (
                            platform.id.name(),
                            platform.create_url(id, self.is_album, Some("album/"), Some("track/")),
                        )
                    }),
                },
            })
            .collect()
    }
}
