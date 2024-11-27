use crate::types::images::Image;
use leptos::*;

#[component]
pub fn SongCard(class: &'static str, title: String, image: Image) -> impl IntoView {
    view! {
        <a class="content-none" href=format!("#{}-link-list", title)>
            <div class=format!("max-w-sm border-2 border-gray-800 shadow mx-[20px] {}", class)>
                <img class="mx-auto p-[20px]" src=image.path width=image.width.unwrap_or("400px".to_owned()) height=image.height.unwrap_or("400px".to_owned()) alt=title.clone()/>
                <h2 class="block text-white text-center text-l font-medium font-sans pb-[20px]">{title}</h2>
            </div>
        </a>
    }
}
