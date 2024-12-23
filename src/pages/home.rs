use leptos::prelude::*;

use crate::components::{card::SongCard, lists::StreamingList, nav};
use crate::config::{CURRENT_SONG, OTHER_SONGS};
use crate::types::links::Song;

#[component]
pub fn Home() -> impl IntoView {
    let active_list = RwSignal::new(Song::<String>::from(CURRENT_SONG));

    view! {
        <nav::Nav/>
        <main class="main" id="home-page">
            <StreamingList list_info=active_list />

            <div class="border border-surface-dark shadow mt-9 pb-9 xl:rounded xl:mx-60" id="previous-releases">
                <span class="flex justify-center mt-9" id="Text">
                    <span class="text-center text-3xl font-semibold font-sans text-transparent bg-clip-text bg-gradient-to-r from-text-dark via-yellow-dark to-lavender-dark">
                        "previous releases"
                    </span>
                </span>

                <ul class="flex flex-wrap justify-center p-5" id="release-buttons">
                    {OTHER_SONGS
                        .iter()
                        .map(|song| {
                            view! {
                                <li>
                                    <SongCard
                                        on:click=move |_| active_list.set((*song).into())
                                        title=song.name.to_string()
                                        image=song.image.into()
                                        class="my-5 ease-in duration-100 md:hover:scale-105"
                                    />
                                </li>
                            }
                        })
                        .collect_view()
                    }
                </ul>
            </div>
        </main>
    }
}
