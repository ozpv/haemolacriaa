use icondata::Icon;
use serde::{Deserialize, Serialize};

use crate::config::USERNAME;

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
    pub fn display(&self) -> &'static str {
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

    pub fn cloned(platform: Platform<&str>) -> Platform<String> {
        match platform {
            Platform::Spotify(x) => Platform::Spotify(x.to_string()),
            Platform::YouTube(x) => Platform::YouTube(x.to_string()),
            Platform::SoundCloud(x) => Platform::SoundCloud(x.to_string()),
            Platform::AppleMusic(x) => Platform::AppleMusic(x.to_string()),
            Platform::Bandcamp(x) => Platform::Bandcamp(x.to_string()),
        }
    }
}
