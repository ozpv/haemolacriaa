use crate::config::{SOCIAL_MEDIA_ITEMS, YEARS_ACTIVE};
use leptos::prelude::*;
use leptos_icons::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-base-dark pt-2 pb-9">
            <div class="flex items-center justify-center">
                <nav class="flex pt-2.5 gap-[8vw] my-2.5 text-sm text-text-dark md:gap-[2vw]">
                    {SOCIAL_MEDIA_ITEMS.iter().map(|item| {
                        item.active.then(|| view! {
                            <a class="p-2 rounded-sm transition-all ease-in duration-75 hover:-translate-y-1 hover:bg-surface-dark-100" href=item.url>
                                <Icon icon=item.icon width="16" height="16" />
                            </a>
                        }.into_any()).unwrap_or(().into_any())
                    }).collect_view()}
                </nav>
            </div>
            <span class="block py-1.5 justify-center text-center text-xs text-surface-dark-200 font-sans">{format!("Copyleft (ɔ) {}-{} ", YEARS_ACTIVE[0], YEARS_ACTIVE[1])}
                <a href="/" class="hover:underline hover:text-sapphire-dark">"haemolacriaa"</a>
                ". All Wrongs Reserved."
            </span>
        </footer>
    }
}
