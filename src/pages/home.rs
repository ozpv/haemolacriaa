use leptos::prelude::*;

use crate::components::{card::SongCard, lists::StreamingList, nav};
use crate::config::{CURRENT_SONG, OTHER_SONGS};
use crate::types::links::Song;

#[component]
pub fn Home() -> impl IntoView {
    let active_list = RwSignal::new(Song::<String>::from(CURRENT_SONG));

    view! {
        <nav::Nav/>
        <div class="bg-base-dark min-h-screen" id="home-page">
            <StreamingList list_info=active_list />

            <div class="border border-surface-dark rounded shadow mt-9 pb-9 xl:mx-60" id="previous-releases">
                <span class="flex justify-center mt-9" id="Text">
                    <span class="text-center text-3xl font-semibold font-sans text-transparent bg-clip-text bg-gradient-to-r from-text-dark via-yellow-dark to-lavender-dark">
                        "Previous Releases"
                    </span>
                </span>

                <div class="flex flex-wrap justify-center p-5" id="release-buttons">
                    {OTHER_SONGS
                        .iter()
                        .map(|song| {
                            let song = Song::<String>::from(*song);
                            view! {
                                <SongCard
                                    on:click=move |_| active_list.set(song.clone())
                                    title=song.name.clone()
                                    image=song.image.clone()
                                    class="md:ease-in md:duration-100 hover:scale-105 md:my-5"
                                />
                            }
                        })
                        .collect_view()
                    }
                </div>
            </div>
        </div>
    }
}
