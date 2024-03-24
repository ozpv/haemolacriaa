use icondata as ico;
use const_format::formatcp;

use crate::types::{
    routes::RouteInfo,
    links::{StreamingPlatform, SocialMediaInfo, SongInfo}, 
    images::Image,
};

/// Information section

// Assumes you only have one, synchronized name.
const USERNAME: &str = "haemolacriaa";

// Every ID is derived from the final id on each profile URL, 
// make sure to ignore tracking info; this is usually contained
// after the a '?' in the url.
const SPOTIFY_ARTIST_ID: &str = "4RzQ0uG5y64uVDj7Az5VkN";

// Apple Music Region and ID
const APPLE_MUSIC_REGION: &str = "us";
const APPLE_MUSIC_ID: &str = "1549699645";

// Youtube channel ID, could also be your '@'
const YOUTUBE_CHANNEL_ID: &str = "UCQDQqA9iaWtlNkwXiCQogYQ";

/// Navbar

// Each route displayed in the navbar.
pub static NAV_ITEMS: [RouteInfo; 2] = [
    RouteInfo {
        name: "Home",
        path: "/",
    },
    RouteInfo {
        name: "Blog",
        path: "/blog",
    },
];

/// Body

// Current song, set it to be displayed on home page.
pub const CURRENT_SONG: SongInfo = EURYDICE_SONG;

// Define any other songs below
pub const EURYDICE_SONG: SongInfo = SongInfo {
    name: "eurydice",
    author: USERNAME,
    image: Image {
        path: "assets/eurydice.webp",
        width: "400px",
        height: "400px",
    },
    spotify_track_id: "3jVgwiRUrfanloK2E1peWf",
    youtube_watch_id: "_qF4fSIdNqs",
    soundcloud_song_id: "eurydice",
    apple_music_album_id: "1707755091",
};

pub static STREAMING_PLATFORMS: [StreamingPlatform; 4] = [
    StreamingPlatform {
        icon: ico::SiSpotify,
        name: "Spotify",
        base_song_url: "https://open.spotify.com/track/",
    },
    StreamingPlatform {
        icon: ico::SiYoutube,
        name: "YouTube",
        base_song_url: "https://www.youtube.com/watch?v=",
    },
    StreamingPlatform {
        icon: ico::SiSoundcloud,
        name: "SoundCloud",
        base_song_url: formatcp!("https://soundcloud.com/{}/", USERNAME),
    },
    StreamingPlatform {
        icon: ico::SiApple,
        name: "Apple Music",
        base_song_url: formatcp!("https://music.apple.com/{}/album/", APPLE_MUSIC_REGION),
    },
];

/// Footer
pub const YEARS_ACTIVE: [&str; 2] = ["2023", "2024"];

// Each item to be displayed on the footer. 
// From left to right.
pub static SOCIAL_MEDIA_ITEMS: [SocialMediaInfo; 5] = [
    SocialMediaInfo {
        icon: ico::SiApple,
        url: formatcp!(
            "https://music.apple.com/{}/artist/{}",
            APPLE_MUSIC_REGION,
            APPLE_MUSIC_ID
        )
    },
    SocialMediaInfo {
        icon: ico::SiSoundcloud,
        url: formatcp!("https://soundcloud.com/{}", USERNAME)
    },
    SocialMediaInfo {
        icon: ico::SiYoutube,
        url: formatcp!("https://youtube.com/channel/{}", YOUTUBE_CHANNEL_ID)
    },
    SocialMediaInfo {
        icon: ico::SiInstagram,
        url: formatcp!("https://instagram.com/{}", USERNAME)
    },
    SocialMediaInfo {
        icon: ico::SiSpotify,
        url: formatcp!("https://open.spotify.com/artist/{}", SPOTIFY_ARTIST_ID)
    }
];
