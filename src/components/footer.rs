use crate::config::{SOCIAL_MEDIA_ITEMS, YEARS_ACTIVE};
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-gray-900 md:p-5">
            <div class="bg-gray-900">
                <hr class="border-gray-800 mx-auto w-full md:w-[90%]" />
                <div class="flex items-center justify-center">
                    <nav class="flex pt-[10px] gap-[8vw] my-[10px] text-sm text-white md:gap-[2vw]">
                        {
                            SOCIAL_MEDIA_ITEMS.iter().map(|item| {
                                if item.active {
                                    view! {
                                        <a class="p-2 rounded-sm transition-all ease-in duration-75 hover:bg-gray-800" href={item.url}><Icon icon={item.icon} width={"16"} height={"16"} /></a>
                                    }.into_view()
                                } else {
                                    ().into_view()
                                }
                            }).collect_view()
                        }
                    </nav>
                </div>
                <span class="block py-[8px] justify-center text-center text-xs text-gray-500 font-sans">{format!("Copyleft (ɔ) {}-{} ", YEARS_ACTIVE[0], YEARS_ACTIVE[1])}
                    <a href={"/"} class="hover:underline hover:text-blue-900">{"haemolacriaa"}</a>{". All Wrongs Reserved."}
                </span>
            </div>
        </footer>
    }
}
