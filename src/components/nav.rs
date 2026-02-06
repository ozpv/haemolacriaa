use crate::config::NAV_ITEMS;
use icondata as ico;
use leptos::{html, prelude::*};
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
        <nav class="bg-black border-gray-200">
            <div class="flex flex-wrap max-w-screen-xl items-center justify-between mx-auto p-[33px]">
                <span class="flex items-center text-xl text-text-dark font-inter">{env!("CARGO_PKG_NAME")}</span>

                <button on:click=toggle_active class="text-sm text-overlay-dark-200 mx-2 md:hidden" aria_label="open navigation menu">
                    <Icon icon=active_icon width="24" height="24"/>
                </button>

                <div class="w-full md:block md:w-auto" class:hidden=hidden_status>
                    <ul class="flex flex-col pl-8 mt-[26px] bg-black md:pl-0 md:flex-row md:mt-0 md:border-0">
                        {NAV_ITEMS
                            .iter()
                            .map(|item| view! {
                                <li class="pb-[26px] md:pb-0" on:click=toggle_active>
                                    {html::a()
                                        .href(item.path)
                                        .class("text-text-dark font-inter pr-10 hover:text-sapphire-dark")
                                        .child(item.name)
                                    }
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
/*
#[component]
pub fn ShopNav() -> impl IntoView {
    let activity = RwSignal::new(false);
    let toggle_active = move |_| activity.update(|status| *status = !*status);

    let active_icon = Signal::derive(move || {
        if activity.get() {
            icondata::BsXLg
        } else {
            icondata::BsList
        }
    });

    let hidden_status = move || !activity.get();

    let shop_count = NodeRef::new();

    #[cfg(feature = "hydrate")]
    {
        use crate::pages::shop::storage::{bag, get_storage};
        use web_sys::HtmlParagraphElement;

        let update_count = move || {
            let count = bag::get_bag_count_or_default(get_storage().as_ref());
            let count_element: HtmlParagraphElement =
                shop_count.get().expect("Shop count element to exist");
            Dom::set_inner_html(&count_element, &count.to_string());
        };

        Effect::new(update_count);

        let storage_handle = window_event_listener(leptos::ev::storage, move |_| update_count());

        on_cleanup(move || storage_handle.remove());
    }

    view! {
        <nav class="bg-black border-gray-200">
            <div class="flex flex-wrap items-center justify-between max-w-screen-xl mx-auto p-4">
                <span class="text-center text-xl text-text-dark font-inter px-5 mt-px">{env!("CARGO_PKG_NAME")}</span>

                <div class="flex md:order-2">
                    <a href="/bag" class="flex flew-row items-center text-text-dark text-sm hover:text-sapphire-dark">
                        <Icon icon={icondata::BsBag} width="16px" height="16px" />
                        <p class="font-inter pl-5 pr-5 md:pr-10" node_ref=shop_count>"0"</p>
                    </a>

                    <button on:click=toggle_active class="text-sm text-overlay-dark-200 md:hidden" aria_label="open navigation menu">
                        <Icon icon=active_icon width="24" height="24"/>
                    </button>
                </div>

                <div class="items-center justify-between w-full md:flex md:w-auto md:order-1" class:hidden=hidden_status>
                    <ul class="flex flex-col pl-8 mt-4 bg-black md:pl-0 md:flex-row md:mt-0 md:border-0">
                        {NAV_ITEMS
                            .iter()
                            .map(|item| view! {
                                <li class="pb-4 md:pb-0" on:click=toggle_active>
                                    {html::a()
                                        .href(item.path)
                                        .class("text-text-dark font-inter pr-10 hover:text-sapphire-dark")
                                        .child(item.name)
                                    }
                                </li>
                            })
                            .collect_view()
                        }
                    </ul>
                </div>
            </div>
        </nav>
    }
}*/
