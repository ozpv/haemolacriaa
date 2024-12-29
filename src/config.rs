use const_format::formatcp;

use crate::types::{
    images::Image,
    links::{
        Platform::{AppleMusic, Bandcamp, SoundCloud, Spotify, YouTube},
        SocialMediaInfo, Song,
    },
    routes::RouteInfo,
};

pub const YEARS_ACTIVE: [&str; 2] = ["2023", "2025"];

/// Information section
// Assumes you only have one, synchronized name.
pub const USERNAME: &str = "haemolacriaa";

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
        name: "home",
        path: "/",
        external: false,
    },
    RouteInfo {
        name: "shop",
        path: "/shop",
        external: true,
    },
];

/// Body
// Current song, set it to be displayed on home page.
pub const CURRENT_SONG: Song<&'static str> = LEAVE_SONG;

// Define any other songs or albums below

pub const LEAVE_SONG: Song<&'static str> = Song {
    name: "leave",
    author: USERNAME,
    image: Image {
        path: "leave.webp",
        width: Some("400"),
        height: Some("400"),
    },
    is_album: false,
    spotify_id: Some(Spotify("5lcsNMrZi4BgbBdMCL1Esl")),
    youtube_id: Some(YouTube("b23ieCaa_fs")),
    soundcloud_id: Some(SoundCloud("leave")),
    apple_music_id: Some(AppleMusic("1765816897")),
    bandcamp_id: None,
    publish_date: chrono::NaiveDate::from_ymd_opt(2024, 9, 13),
};

pub static OTHER_SONGS: [Song<&'static str>; 3] = [
    Song {
        name: "swarm",
        author: USERNAME,
        image: Image {
            path: "swarm.webp",
            width: Some("400"),
            height: Some("400"),
        },
        is_album: false,
        spotify_id: Some(Spotify("7aLZxnXz0ZTkOJOlVmeqZQ")),
        youtube_id: Some(YouTube("dbLlehkojoI")),
        soundcloud_id: Some(SoundCloud("swarm")),
        apple_music_id: Some(AppleMusic("1756360277")),
        bandcamp_id: None,
        publish_date: chrono::NaiveDate::from_ymd_opt(2024, 7, 19),
    },
    Song {
        name: "haemolacriaa",
        author: USERNAME,
        image: Image {
            path: "haemolacriaa.webp",
            width: Some("400"),
            height: Some("400"),
        },
        is_album: true,
        spotify_id: Some(Spotify("5TmWqQ0YoJ9t8PHPSqhZLp")),
        youtube_id: Some(YouTube("OLAK5uy_k5kigsMsg7pFfb3_J566qnxftni1ba7jw")),
        soundcloud_id: Some(SoundCloud("haemolacriaa")),
        apple_music_id: Some(AppleMusic("1739982491")),
        bandcamp_id: Some(Bandcamp("haemolacriaa")),
        publish_date: chrono::NaiveDate::from_ymd_opt(2024, 4, 19),
    },
    Song {
        name: "stay",
        author: USERNAME,
        image: Image {
            path: "stay.webp",
            width: Some("400"),
            height: Some("400"),
        },
        is_album: false,
        spotify_id: Some(Spotify("3rzuDN6mPujdByy2QfCArW")),
        youtube_id: Some(YouTube("b_IEnLvLn3Y")),
        soundcloud_id: Some(SoundCloud("stay")),
        apple_music_id: Some(AppleMusic("1681486735")),
        bandcamp_id: None,
        publish_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 14),
    },
];

// Each item to be displayed on the footer.
// From left to right.
pub static SOCIAL_MEDIA_ITEMS: [SocialMediaInfo; 8] = [
    SocialMediaInfo {
        icon: icondata::SiGithub,
        url: formatcp!("https://github.com/{GITHUB_USERNAME}"),
        active: true,
    },
    SocialMediaInfo {
        icon: icondata::SiTiktok,
        url: formatcp!("https://tiktok.com/@{TIKTOK_USERNAME}"),
        active: true,
    },
    SocialMediaInfo {
        icon: icondata::SiYoutube,
        url: formatcp!("https://youtube.com/channel/{YOUTUBE_CHANNEL_ID}"),
        active: true,
    },
    SocialMediaInfo {
        icon: icondata::SiInstagram,
        url: formatcp!("https://instagram.com/{USERNAME}"),
        active: true,
    },
    SocialMediaInfo {
        icon: icondata::SiApple,
        url: formatcp!("https://music.apple.com/{APPLE_MUSIC_REGION}/artist/{APPLE_MUSIC_ID}",),
        active: false,
    },
    SocialMediaInfo {
        icon: icondata::SiSoundcloud,
        url: formatcp!("https://soundcloud.com/{USERNAME}"),
        active: false,
    },
    SocialMediaInfo {
        icon: icondata::SiSpotify,
        url: formatcp!("https://open.spotify.com/artist/{SPOTIFY_ARTIST_ID}"),
        active: false,
    },
    SocialMediaInfo {
        icon: icondata::SiBandcamp,
        url: formatcp!("https://{USERNAME}.bandcamp.com/"),
        active: false,
    },
];
