use crate::config::NAV_ITEMS;
use icondata as ico;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Nav() -> impl IntoView {
    let activity = RwSignal::new(false);
    let toggle_active = move |_| activity.update(|status| *status = !*status);

    let active_icon = Signal::derive(move || {
        if activity.get() {
            ico::BsXLg
        } else {
            ico::BsList
        }
    });

    let hidden_status = move || !activity.get();

    view! {
        <nav class="bg-base-dark border-gray-200">
            <div class="flex flex-wrap max-w-screen-xl items-center justify-between mx-auto p-4">
                <span class="flex items-center text-xl text-text-dark font-inter px-5">{env!("CARGO_PKG_NAME")}</span>

                <button on:click=toggle_active class="text-sm text-overlay-dark-200 mx-2 md:hidden">
                    <Icon icon=active_icon width="24" height="24"/>
                </button>

                <div class="w-full md:block md:w-auto" class:hidden=hidden_status id="navbar-default">
                    <ul class="font-inter font-medium flex flex-col p-4 md:p-0 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-base-dark">
                        {NAV_ITEMS
                            .iter()
                            .map(|item| view! {
                                <li class="p-2" on:click=toggle_active>
                                    <a href=item.path class="text-text-dark font-inter p-2.5 hover:bg-base-dark md:hover:text-sapphire-dark md:hover:bg-transparent">
                                        {item.name}
                                    </a>
                                </li>
                            })
                            .collect_view()
                        }
                    </ul>
                </div>
            </div>
        </nav>
    }
}
