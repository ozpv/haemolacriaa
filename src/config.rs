use const_format::formatcp;
use icondata as ico;

use crate::types::{
    images::Image,
    links::{SocialMediaInfo, Song, StreamingPlatform},
    routes::RouteInfo,
};
use PlatformId::{AppleMusic, Bandcamp, SoundCloud, Spotify, YouTube};

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
pub const CURRENT_SONG: Song<&'static str> = LEAVE_SONG;

// Define any other songs or albums below

pub const LEAVE_SONG: Song<&'static str> = Song {
    name: "leave",
    author: USERNAME,
    image: Image {
        path: "leave.webp",
        width: Some("400px"),
        height: Some("400px"),
    },
    is_album: false,
    spotify_id: Some("5lcsNMrZi4BgbBdMCL1Esl"),
    youtube_id: Some("b23ieCaa_fs"),
    soundcloud_id: Some("leave"),
    apple_music_id: Some("1765816897"),
    bandcamp_id: None,
    publish_date: chrono::NaiveDate::from_ymd_opt(2024, 9, 13),
};

pub static OTHER_SONGS: [Song<&'static str>; 3] = [
    Song {
        name: "swarm",
        author: USERNAME,
        image: Image {
            path: "swarm.webp",
            width: Some("400px"),
            height: Some("400px"),
        },
        is_album: false,
        spotify_id: Some("7aLZxnXz0ZTkOJOlVmeqZQ"),
        youtube_id: Some("dbLlehkojoI"),
        soundcloud_id: Some("swarm"),
        apple_music_id: Some("1756360277"),
        bandcamp_id: None,
        publish_date: chrono::NaiveDate::from_ymd_opt(2024, 7, 19),
    },
    Song {
        name: "haemolacriaa",
        author: USERNAME,
        image: Image {
            path: "haemolacriaa.webp",
            width: Some("400px"),
            height: Some("400px"),
        },
        is_album: true,
        spotify_id: Some("5TmWqQ0YoJ9t8PHPSqhZLp"),
        youtube_id: Some("OLAK5uy_k5kigsMsg7pFfb3_J566qnxftni1ba7jw"),
        soundcloud_id: Some("haemolacriaa"),
        apple_music_id: Some("1739982491"),
        bandcamp_id: Some("haemolacriaa"),
        publish_date: chrono::NaiveDate::from_ymd_opt(2024, 4, 19),
    },
    Song {
        name: "stay",
        author: USERNAME,
        image: Image {
            path: "stay.webp",
            width: Some("400px"),
            height: Some("400px"),
        },
        is_album: false,
        spotify_id: Some("3rzuDN6mPujdByy2QfCArW"),
        youtube_id: Some("b_IEnLvLn3Y"),
        soundcloud_id: Some("stay"),
        apple_music_id: Some("1681486735"),
        bandcamp_id: None,
        publish_date: chrono::NaiveDate::from_ymd_opt(2023, 4, 14),
    },
];

/// (x, y) x: the name of the platform, y: the base link to a song
/// Used T to save myself from writing &'static str and your eyes
pub enum PlatformId<T = &'static str> {
    Spotify(T, T),
    YouTube(T, T),
    SoundCloud(T, T),
    AppleMusic(T, T),
    Bandcamp(T, T),
}

impl<T> PlatformId<T> {
    pub fn unwrap_name(&self) -> &T {
        match self {
            Spotify(x, _)
            | YouTube(x, _)
            | SoundCloud(x, _)
            | AppleMusic(x, _)
            | Bandcamp(x, _) => x,
        }
    }
    pub fn unwrap_link(&self) -> &T {
        match self {
            Spotify(_, y)
            | YouTube(_, y)
            | SoundCloud(_, y)
            | AppleMusic(_, y)
            | Bandcamp(_, y) => y,
        }
    }
}

pub static STREAMING_PLATFORMS: [StreamingPlatform; 5] = [
    StreamingPlatform {
        icon: ico::SiSpotify,
        id: PlatformId::Spotify("Spotify", "https://open.spotify.com/"),
    },
    StreamingPlatform {
        icon: ico::SiYoutube,
        id: PlatformId::YouTube("YouTube", "https://www.youtube.com/"),
    },
    StreamingPlatform {
        icon: ico::SiSoundcloud,
        id: PlatformId::SoundCloud(
            "SoundCloud",
            formatcp!("https://soundcloud.com/{}/", USERNAME),
        ),
    },
    StreamingPlatform {
        icon: ico::SiApple,
        id: PlatformId::AppleMusic(
            "Apple Music",
            formatcp!("https://music.apple.com/{}/album/", APPLE_MUSIC_REGION),
        ),
    },
    StreamingPlatform {
        icon: ico::SiBandcamp,
        id: PlatformId::Bandcamp("Bandcamp", formatcp!("https://{}.bandcamp.com/", USERNAME)),
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
