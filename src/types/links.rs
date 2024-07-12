use icondata::Icon;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

use crate::config::{PlatformId, PlatformId::*, STREAMING_PLATFORMS};
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
    fn create_url<'a, T>(&self, id: &T, is_album: bool, main: &'a str, alt: &'a str) -> String
    where
        T: std::fmt::Display,
    {
        format!(
            "{}{}{}",
            &self.id.unwrap_link(),
            if is_album { &main } else { &alt },
            &id
        )
    }
}

pub struct StreamingInfo {
    pub song_id: Option<(&'static str, String)>,
    pub platform_icon: Icon,
}

// Song info section

#[derive(Copy, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Song<T = String> {
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
            name: s.name.to_owned(),
            author: s.author.to_owned(),
            image: Image::from(s.image),
            is_album: s.is_album,
            spotify_id: s.spotify_id.map(|st| st.to_owned()),
            youtube_id: s.youtube_id.map(|st| st.to_owned()),
            soundcloud_id: s.soundcloud_id.map(|st| st.to_owned()),
            apple_music_id: s.apple_music_id.map(|st| st.to_owned()),
            bandcamp_id: s.bandcamp_id.map(|st| st.to_owned()),
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
                    Spotify(x, _) => self.spotify_id.as_ref().map(|id| {
                        (
                            x,
                            platform.create_url(id, self.is_album, "album/", "track/"),
                        )
                    }),
                    YouTube(x, _) => self.youtube_id.as_ref().map(|id| {
                        (
                            x,
                            platform.create_url(id, self.is_album, "playlist?list=", "watch?v="),
                        )
                    }),
                    SoundCloud(x, _) => self
                        .soundcloud_id
                        .as_ref()
                        .map(|id| (x, platform.create_url(id, self.is_album, "sets/", ""))),
                    AppleMusic(x, _) => self
                        .apple_music_id
                        .as_ref()
                        .map(|id| (x, platform.create_url(id, true, "", ""))),
                    Bandcamp(x, _) => self.bandcamp_id.as_ref().map(|id| {
                        (
                            x,
                            platform.create_url(id, self.is_album, "album/", "track/"),
                        )
                    }),
                },
            })
            .collect()
    }
}
