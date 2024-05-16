use crate::types::links::SongInfo;
use leptos::*;

#[component]
pub fn SongCard<'a>(class: &'a str, song_info: &'static SongInfo) -> impl IntoView {
    view! {
        <div class=format!("max-w-sm border-2 border-gray-800 shadow mx-[20px] {}", class)>
            <img class="mx-auto p-[20px]" src=song_info.image.path width=song_info.image.width height=song_info.image.height alt=song_info.name/>
            <h2 class="block text-white text-center text-l font-medium font-sans pb-[20px]">{song_info.name}</h2>
            <a 
                href=format!("#{}-links", song_info.name) 
                class="static inset-y-0 left-0 w-full"
            ></a>
        </div>
    }
}
