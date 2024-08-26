use leptos::*;
use leptos_router::*;

use crate::components::lists::StreamingList;
use crate::song_db::get_song_by_name;

#[component]
pub fn Releases() -> impl IntoView {
    let name = move || use_params_map().with(|params| params.get("name").cloned());
    let name = create_memo(move |_| name().unwrap().clone());

    let song_info = create_resource(move || name.get(), get_song_by_name);

    view! {
        <div class="min-h-screen bg-gray-900" id="release-page">
            <Transition fallback=move || view! { <p class="text-sm font-medium text-white">{"Loading..."}</p> }>
                {move || song_info.get().map(|song| match song {
                        Ok(song) => view!{
                            <StreamingList list_info=song.into()/>
                        }.into_view(),
                        Err(_) => view! {
                            <p class="text-sm font-medium text-white">{
                                if name.get() == "" {
                                    "Error loading a release! Please supply a name in the URL.".to_owned()
                                } else {
                                    format!("Error loading release with name {}!", name.get())
                                }
                            }</p>
                        }.into_view(),
                    }
                )}
            </Transition>
        </div>
    }
}
