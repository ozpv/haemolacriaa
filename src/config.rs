use const_format::formatcp;
use icondata as ico;

use crate::types::{
    images::Image,
    links::{SocialMediaInfo, SongInfo, StreamingPlatform},
    routes::RouteInfo,
};

/// Information section

// Assumes you only have one, synchronized name.
const USERNAME: &str = "haemolacriaa";

// Add other users here
const TIKTOK_USERNAME: &str = "haemolacriaamusic";
const GITHUB_USERNAME: &str = "ozpv";

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
        name: "Shop",
        path: "/shop",
    },
];

/// Body

// Current song, set it to be displayed on home page.
pub const CURRENT_SONG: SongInfo = EURYDICE_SONG;

// Define any other songs or albums below

pub const HAEMOLACRIAA_ALBUM: SongInfo = SongInfo {
    name: "haemolacriaa",
    author: USERNAME,
    image: Image {
        path: "assets/haemolacriaa.webp",
        width: "400px",
        height: "400px",
    },
    is_album: true,
    spotify_id: "",
    youtube_id: "",
    soundcloud_id: "haemolacriaa",
    apple_music_id: "",
    bandcamp_id: "haemolacriaa",
};

pub const EURYDICE_SONG: SongInfo = SongInfo {
    name: "eurydice",
    author: USERNAME,
    image: Image {
        path: "assets/eurydice.webp",
        width: "400px",
        height: "400px",
    },
    is_album: false,
    spotify_id: "3jVgwiRUrfanloK2E1peWf",
    youtube_id: "_qF4fSIdNqs",
    soundcloud_id: "eurydice",
    apple_music_id: "1707755091",
    bandcamp_id: "eurydice",
};

pub static STREAMING_PLATFORMS: [StreamingPlatform; 5] = [
    StreamingPlatform {
        icon: ico::SiSpotify,
        name: "Spotify",
        base_song_url: "https://open.spotify.com/",
    },
    StreamingPlatform {
        icon: ico::SiYoutube,
        name: "YouTube",
        base_song_url: "https://www.youtube.com/",
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
    StreamingPlatform {
        icon: ico::SiBandcamp,
        name: "Bandcamp",
        base_song_url: formatcp!("https://{}.bandcamp.com/", USERNAME),
    },
];

/// Footer
pub const YEARS_ACTIVE: [&str; 2] = ["2023", "2024"];

// Each item to be displayed on the footer.
// From left to right.
pub static SOCIAL_MEDIA_ITEMS: [SocialMediaInfo; 8] = [
    SocialMediaInfo {
        icon: ico::SiGithub,
        url: formatcp!("https://github.com/{}", GITHUB_USERNAME),
        active: true,
    },
    SocialMediaInfo {
        icon: ico::SiTiktok,
        url: formatcp!("https://tiktok.com/@{}", TIKTOK_USERNAME),
        active: true,
    },
    SocialMediaInfo {
        icon: ico::SiYoutube,
        url: formatcp!("https://youtube.com/channel/{}", YOUTUBE_CHANNEL_ID),
        active: true,
    },
    SocialMediaInfo {
        icon: ico::SiInstagram,
        url: formatcp!("https://instagram.com/{}", USERNAME),
        active: true,
    },
    SocialMediaInfo {
        icon: ico::SiApple,
        url: formatcp!(
            "https://music.apple.com/{}/artist/{}",
            APPLE_MUSIC_REGION,
            APPLE_MUSIC_ID
        ),
        active: false,
    },
    SocialMediaInfo {
        icon: ico::SiSoundcloud,
        url: formatcp!("https://soundcloud.com/{}", USERNAME),
        active: false,
    },
    SocialMediaInfo {
        icon: ico::SiSpotify,
        url: formatcp!("https://open.spotify.com/artist/{}", SPOTIFY_ARTIST_ID),
        active: false,
    },
    SocialMediaInfo {
        icon: ico::SiBandcamp,
        url: formatcp!("https://{}.bandcamp.com/", USERNAME),
        active: false,
    },
];
