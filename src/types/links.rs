use icondata::Icon;

use crate::config::STREAMING_PLATFORMS;
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
    pub name: &'static str,
    pub base_song_url: &'static str,
}

pub struct StreamingInfo {
    pub song_url: String,
    pub platform_name: String,
    pub platform_icon: Icon,
}

/// Song info section

pub struct SongInfo {
    pub name: &'static str,
    pub author: &'static str,
    pub image: Image,
    pub is_album: bool,
    pub spotify_id: &'static str,
    pub youtube_id: &'static str,
    pub soundcloud_id: &'static str,
    pub apple_music_id: &'static str,
    pub bandcamp_id: &'static str,
}

impl SongInfo {
    pub fn build_streaming_info(&self) -> Vec<StreamingInfo> {
        STREAMING_PLATFORMS
            .iter()
            .map(|platform| {
                StreamingInfo {
                    platform_name: platform.name.to_owned(),
                    platform_icon: platform.icon,
                    song_url: format!("{}{}", platform.base_song_url, {
                        // Unfortunately, you must add new platforms here,
                        // unless I come up with a better algorithm :(
                        match platform.name {
                            "Spotify" => format!("{}{}", { 
                                if self.is_album {
                                    "album/"
                                } else {
                                    "track/"
                                }
                            }, self.spotify_id),
                            "YouTube" => format!("{}{}", {
                                if self.is_album {
                                    "playlist?list="
                                } else {
                                    "watch?v="
                                }
                            }, self.youtube_id),
                            "SoundCloud" => format!("{}{}", {
                                if self.is_album {
                                    "sets/"
                                } else {
                                    ""
                                }
                            }, self.soundcloud_id), 
                            "Apple Music" => self.apple_music_id.to_string(),
                            "Bandcamp" => format!("{}{}", { 
                                if self.is_album {
                                    "album/"
                                } else {
                                    "track/"
                                }
                            }, self.bandcamp_id), 
                            _ => "/".to_string(),
                        }
                    }),
                }
            })
            .collect()
    }
}
