use leptos::prelude::*;
use leptos_meta::Meta;

use crate::components::{card::SongCard, lists::StreamingList, nav::Nav};
use crate::config::{CURRENT_SONG, OTHER_SONGS};

#[component]
pub fn Home() -> impl IntoView {
    let active_list = RwSignal::new(CURRENT_SONG);

    view! {
        <Nav/>
        <Meta name="description" content="leave by haemolacriaa is out now" />
        <main class="main">
            <StreamingList list_info=active_list />

            <div class="border border-surface-dark shadow mt-9 pb-9 xl:rounded xl:mx-60" aria_label="previous song releases">
                <span class="flex justify-center mt-9">
                    <span class="text-center text-3xl font-semibold font-sans text-transparent bg-clip-text bg-gradient-to-r from-text-dark via-yellow-dark to-lavender-dark">
                        "previous releases"
                    </span>
                </span>

                <ul class="flex flex-wrap justify-center p-5" aria_label="release buttons">
                    {OTHER_SONGS
                        .iter()
                        .map(|song| {
                            view! {
                                <li>
                                    <SongCard
                                        on:click=move |_| active_list.set(*song)
                                        title=song.name
                                        image=song.image
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
