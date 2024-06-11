use leptos::*;

use crate::components::{card::SongCard, lists::StreamingList, misc::Divider};
use crate::config::{CURRENT_SONG, OTHER_SONGS};

#[component]
pub fn Home() -> impl IntoView {
    let (list, set_list) = create_signal((&CURRENT_SONG, format!("{}-link-list", &CURRENT_SONG.name)));

    view! {
        <div class="bg-gray-900 min-h-screen" id="home page">
            <StreamingList list_info=list />
            <Divider/>
            <div class="" id="previous-releases">
                <span class="flex justify-center mt-[30px]" id="Text">
                    <span class="text-center text-3xl font-semibold font-sans text-transparent bg-clip-text bg-gradient-to-r from-white via-yellow-900 to-purple-900">
                        {"Previous Releases"}
                    </span>
                </span>
                <div class="flex flex-wrap justify-center p-[20px]" id="release-buttons">
                    {
                        OTHER_SONGS
                            .iter()
                            .map(|song| {
                               view! {
                                       <SongCard
                                           on:click=move |_| { 
                                               set_list.set((song, format!("{}-link-list", song.name)));
                                           }
                                           song_info=song
                                           class="ease-in duration-100 hover:scale-105 my-[20px]"
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
