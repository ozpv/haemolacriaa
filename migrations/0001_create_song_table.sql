CREATE TYPE image AS (
	path TEXT,
	width TEXT,
	height TEXT
);

CREATE TABLE IF NOT EXISTS song (
	name TEXT NOT NULL,
	author TEXT NOT NULL,
	song_image IMAGE,
	is_album BOOLEAN NOT NULL,
	spotify_id TEXT,
	youtube_id TEXT,
	soundcloud_id TEXT,
	apple_music_id TEXT,
	bandcamp_id TEXT,
	publish_date DATE NOT NULL
);
