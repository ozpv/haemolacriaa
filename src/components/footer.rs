use crate::config::{SOCIAL_MEDIA_ITEMS, YEARS_ACTIVE};
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="sticky top-full bg-gray-900 md:p-5">
            <div class="bg-gray-950 shadow md:rounded-lg">
                <div class="flex items-center justify-center">
                    <nav class="flex pt-[10px] gap-[8vw] mt-3 text-sm text-white md:gap-[2vw]">
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
                <hr class="my-[16px] border-gray-800 mx-auto w-full md:w-[70%] md:my-[20px]" />
                <span class="block pb-[20px] justify-center text-center text-xs text-gray-500 font-sans md:pb-[16px]">{format!("Copyleft (ɔ) {}-{} ", YEARS_ACTIVE[0], YEARS_ACTIVE[1])}
                    <a href={"/"} class="hover:underline hover:text-blue-900">{"haemolacriaa"}</a>{". All Wrongs Reserved."}
                </span>
            </div>
        </footer>
    }
}
