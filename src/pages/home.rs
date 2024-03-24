use leptos::*;

use crate::components::lists::StreamingList;
use crate::config::{CURRENT_SONG, SOCIAL_MEDIA_ITEMS};

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="bg-gray-900 min-h-screen" id="home page">
            <StreamingList song_info=&CURRENT_SONG id="streaming-link-list" appendix=true appendix_social=&SOCIAL_MEDIA_ITEMS[3]/>
        </div>
    }
}
