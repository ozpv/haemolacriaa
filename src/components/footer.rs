use crate::config::{SOCIAL_MEDIA_ITEMS, YEARS_ACTIVE};
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-black pt-2 pb-9">
            <div class="flex items-center justify-center">
                <nav class="flex pt-2.5 gap-[8vw] my-2.5 text-sm text-text-dark font-inter md:gap-[2vw]">
                    {SOCIAL_MEDIA_ITEMS.iter().map(|item| {
                        if item.active {
                            view! {
                                <a class="p-2 rounded-sm transition-all ease-in duration-75 hover:-translate-y-1 hover:bg-surface-dark-100" href=item.url>
                                    <Icon icon=item.icon width="16" height="16" />
                                </a>
                            }.into_any()
                        } else {
                            ().into_any()
                        }
                    }).collect_view()}
                </nav>
            </div>
            <span class="block justify-center text-center text-xs text-surface-dark-200 font-inter py-1.5">{format!("copyleft (ɔ) {}-{} ", YEARS_ACTIVE[0], YEARS_ACTIVE[1])}
                <a href="/" class="hover:underline hover:text-sapphire-dark">"haemolacriaa"</a>
                ". all wrongs reserved."
            </span>
        </footer>
    }
}
