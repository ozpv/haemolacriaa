use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use std::collections::HashMap;
use std::sync::LazyLock;

use crate::app::NotFound;
use crate::components::{lists::StreamingList, nav};
use crate::config::{CURRENT_SONG, OTHER_SONGS};
use crate::types::song::Song;

#[component]
pub fn Releases() -> impl IntoView {
    static SONGS: LazyLock<HashMap<String, Song>> = LazyLock::new(|| {
        let mut set = HashMap::with_capacity(OTHER_SONGS.len() + 1);

        // insert the latest at the top
        let song_name = CURRENT_SONG.name.to_string().to_ascii_lowercase();
        set.insert(song_name, CURRENT_SONG);

        for song in &OTHER_SONGS {
            let song_name = song.name.to_string().to_ascii_lowercase();
            set.insert(song_name, *song);
        }

        set
    });

    let name = move || use_params_map().with(|params| params.get("name"));
    let name = Memo::new(move |_| name().unwrap());

    // ensure there's no flicker by doing this on the server
    let songs = move || {
        Suspend::new(async move {
            if let Some(song) = SONGS.get(&name.get()) {
                view! {
                    <nav::Nav />
                    <main class="main">
                        <StreamingList list_info=*song />
                    </main>
                }
                .into_any()
            } else {
                NotFound.into_any()
            }
        })
    };

    view! {
        <Suspense fallback=|| ()>
            {songs}
        </Suspense>
    }
}
