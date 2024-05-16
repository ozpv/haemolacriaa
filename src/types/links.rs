use icondata::Icon;

use crate::config::{PlatformId, PlatformId::*, STREAMING_PLATFORMS};
use crate::types::images::Image;

/// Social media info section

#[derive(PartialEq)]
pub struct SocialMediaInfo {
    pub icon: Icon,
    pub url: &'static str,
    pub active: bool,
}

/// Streaming platform section

pub struct StreamingPlatform {
    pub icon: Icon,
    pub id: PlatformId<&'static str>,
}

pub struct StreamingInfo {
    pub song_id: Option<(&'static str, String)>,
    pub platform_icon: Icon,
}

/// Song info section

pub struct SongInfo {
    pub name: &'static str,
    pub author: &'static str,
    pub image: Image,
    pub is_album: bool,
    pub spotify_id: Option<&'static str>,
    pub youtube_id: Option<&'static str>,
    pub soundcloud_id: Option<&'static str>,
    pub apple_music_id: Option<&'static str>,
    pub bandcamp_id: Option<&'static str>,
}

impl StreamingPlatform {
    fn create_url<'a>(&self, id: &'a str, is_album: bool, main: &'a str, alt: &'a str) -> String {
        format!("{}{}{}", &self.id.unwrap_link(), if is_album { &main } else { &alt }, &id)
    }
}

impl SongInfo {
    pub fn build_streaming_info(&self) -> Vec<StreamingInfo> {
        STREAMING_PLATFORMS
            .iter()
            .map(|platform| StreamingInfo {
                platform_icon: platform.icon,
                song_id: match platform.id {
                    Spotify(x, _) => {
                        if let Some(id) = &self.spotify_id {
                            Some((x, platform.create_url(id, self.is_album, "album/", "track/")))
                        } else {
                            None
                        }
                    }
                    YouTube(x, _) => {
                        if let Some(id) = &self.youtube_id {
                            Some((x, platform.create_url(id, self.is_album, "playlist?list=", "watch?v=")))
                        } else {
                            None
                        }
                    }
                    SoundCloud(x, _) => {
                        if let Some(id) = &self.soundcloud_id {
                            Some((x, platform.create_url(id, self.is_album, "sets/", "")))
                        } else {
                            None
                        }
                    }
                    AppleMusic(x, _) => {
                        if let Some(id) = &self.apple_music_id {
                            Some((x, platform.create_url(id, true, "", "")))
                        } else {
                            None
                        }
                    }
                    Bandcamp(x, _) => {
                        if let Some(id) = &self.bandcamp_id {
                            Some((x, platform.create_url(id, self.is_album, "album/", "track/")))
                        } else {
                            None
                        }
                    }
                },
            })
            .collect()
    }
}
