use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::{lists::StreamingList, nav};
use crate::song_db::get_song_by_name;

#[component]
pub fn Releases() -> impl IntoView {
    let name = move || use_params_map().with(|params| params.get("name"));
    let name = Memo::new(move |_| name().unwrap());

    let song_info = Resource::new(move || name.get(), get_song_by_name);

    view! {
        <nav::Nav />
        <main class="main" id="release-page">
            <Transition fallback=move || view! { <p class="text-sm font-medium text-text-dark">{"Loading..."}</p> }>
                {move || song_info.get().map(|song| match song {
                        Ok(song) => view!{
                            <StreamingList list_info=RwSignal::new(song)/>
                        }.into_any(),
                        Err(_) => view! {
                            <p class="text-sm font-medium text-text-dark">{
                                if name.get() == "" {
                                    "Error loading a release! Please supply a name in the URL.".to_owned()
                                } else {
                                    format!("Error loading release with name {}!", name.get())
                                }
                            }</p>
                        }.into_any(),
                    }
                )}
            </Transition>
        </main>
    }
}
