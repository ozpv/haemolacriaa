use leptos::*;

use crate::components::{card::SongCard, lists::StreamingList, misc::Divider};
use crate::config::{CURRENT_SONG, OTHER_SONGS};
use crate::song_db::{get_latest_release, get_range_of_songs};
use crate::types::links::Song;

#[component]
pub fn Home() -> impl IntoView {
    let active_list = create_rw_signal(Song::<String>::from(CURRENT_SONG));

    view! {
        <div class="bg-gray-900 min-h-screen" id="home page">

            <StreamingList list_info=active_list/>

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
                            .map(|song| Song::<String>::from(*song))
                            .map(|song| {
                                view! {
                                    <SongCard
                                        on:click=move |_| {
                                            active_list.set(song.clone());
                                        }
                                        title=song.name.clone()
                                        image=song.image.clone()
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
