use crate::types::images::Image;
use leptos::prelude::*;

#[component]
pub fn SongCard(class: &'static str, title: String, image: Image) -> impl IntoView {
    view! {
        <a class="content-none" href=format!("#{}-link-list", &title)>
            <div class=format!("max-w-sm border rounded border-surface-dark shadow mx-5 hover:bg-surface-dark {}", class)>
                <img class="text-text-dark mx-auto p-5" src=image.path width="400px" height="400px" alt=title.clone()/>
                <h2 class="block text-text-dark text-center text-l font-medium font-inter pb-5">{title.clone()}</h2>
            </div>
        </a>
    }
}
