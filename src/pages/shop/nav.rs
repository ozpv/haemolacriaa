use leptos::{html, prelude::*};
use leptos_icons::Icon;

use crate::config::NAV_ITEMS;

#[component]
pub fn Nav() -> impl IntoView {
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
        use super::storage::{get_storage, bag};
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
        <nav class="bg-base-dark border-gray-200">
            <div class="flex flex-wrap items-center justify-between max-w-screen-xl mx-auto p-4">
                <span class="text-center text-xl text-text-dark font-inter px-5">{env!("CARGO_PKG_NAME")}</span>

                <div class="flex items-center">
                    <div class="hidden w-full md:flex md:w-auto md:order-2" class:hidden=hidden_status>
                        <ul class="flex flex-col bg-base-dark font-inter mt-6 md:flex-row md:mt-0">
                            {NAV_ITEMS
                                .iter()
                                .map(|item| view! {
                                    <li class="p-2" on:click=toggle_active>
                                        {html::a()
                                            .href(item.path)
                                            .class("text-text-dark font-inter p-2.5 md:hover:text-sapphire-dark")
                                            .rel(item.external.then_some("external"))
                                            .inner_html(item.name)
                                        }
                                    </li>
                                })
                                .collect_view()
                            }
                        </ul>
                    </div>

                    <div class="flex md:order-1">
                        <a href="/bag" class="flex flew-row items-center text-text-dark text-sm mx-2 hover:text-sapphire-dark">
                            <Icon icon={icondata::BsBag} width="16px" height="16px" />
                            <p class="font-inter mx-3" node_ref=shop_count>"0"</p>
                        </a>

                        <button on:click=toggle_active class="text-sm text-overlay-dark-200 mx-2 md:hidden">
                            <Icon icon=active_icon width="24px" height="24px"/>
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}
