use icondata::Icon;
use serde::{Serialize,Deserialize};

use crate::config::{PlatformId, PlatformId::*, STREAMING_PLATFORMS};
use crate::types::images::{ConstImage, Image};

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
    fn create_url<'a>(&self, id: &String, is_album: bool, main: &'a str, alt: &'a str) -> String {
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Song {
    pub name: String,
    pub author: String,
    pub image: Image,
    pub is_album: bool,
    pub spotify_id: Option<String>,
    pub youtube_id: Option<String>,
    pub soundcloud_id: Option<String>,
    pub apple_music_id: Option<String>,
    pub bandcamp_id: Option<String>,
}

impl Song {
   pub fn get_key(&self) -> String {
        format!(
            "{}{}", 
            self.name,
            if self.is_album {
                "album"
            } else {
                "song"
            },
        )
   }
}

pub struct ConstSong {
    pub name: &'static str,
    pub author: &'static str,
    pub image: ConstImage,
    pub is_album: bool,
    pub spotify_id: Option<&'static str>,
    pub youtube_id: Option<&'static str>,
    pub soundcloud_id: Option<&'static str>,
    pub apple_music_id: Option<&'static str>,
    pub bandcamp_id: Option<&'static str>,
}

impl ConstSong {
    pub fn to_song(&self) -> Song {
        Song {
            name: self.name.to_string(),
            author: self.author.to_string(),
            image: self.image.to_image(),
            is_album: self.is_album,
            spotify_id: self.spotify_id.map(|s| s.to_string()),
            youtube_id: self.youtube_id.map(|s| s.to_string()),
            soundcloud_id: self.soundcloud_id.map(|s| s.to_string()),
            apple_music_id: self.apple_music_id.map(|s| s.to_string()),
            bandcamp_id: self.bandcamp_id.map(|s| s.to_string()),
        }
    }
}

impl Song {
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
                        .as_ref().map(|id| (x, platform.create_url(id, self.is_album, "sets/", ""))),
                    AppleMusic(x, _) => self
                        .apple_music_id
                        .as_ref().map(|id| (x, platform.create_url(id, true, "", ""))),
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
