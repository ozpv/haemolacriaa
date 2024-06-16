use crate::types::links::Song;
use leptos::*;

#[component]
pub fn SongCard(class: &'static str, song_info: Song) -> impl IntoView {
    view! {
        <a class="content-none" href=format!("#{}-link-list", song_info.name)>
            <div class=format!("max-w-sm border-2 border-gray-800 shadow mx-[20px] {}", class)>
                <img class="mx-auto p-[20px]" src=song_info.image.path width=song_info.image.width height=song_info.image.height alt=song_info.name.clone()/>
                <h2 class="block text-white text-center text-l font-medium font-sans pb-[20px]">{song_info.name}</h2>
            </div>
        </a>
    }
}
