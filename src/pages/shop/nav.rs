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
        use super::storage::{get_storage, Bag};
        use web_sys::HtmlParagraphElement;

        let update_count = move || {
            let count = Bag::get_bag_count_or_default(get_storage().as_ref());
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
            <div class="flex flex-wrap max-w-screen-xl items-center justify-between mx-auto p-4">
                <span class="text-center text-xl text-text-dark font-inter px-5">{env!("CARGO_PKG_NAME")}</span>

                <div class="w-full md:block md:w-auto" class:hidden=hidden_status id="navbar-default">
                    <ul class="flex flex-col p-4 md:p-0 mt-4 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-base-dark">
                        {NAV_ITEMS
                            .iter()
                            .map(|item| view! {
                                <li class="p-2" on:click=toggle_active>
                                    {html::a()
                                        .href(item.path)
                                        .class("text-text-dark font-inter p-2.5 hover:bg-base-dark md:hover:text-sapphire-dark md:hover:bg-transparent")
                                        .rel(item.external.then_some("external"))
                                        .inner_html(item.name)
                                    }
                                </li>
                            })
                            .collect_view()
                        }
                    </ul>
                </div>

                <a href="/bag" class="flex flew-row text-text-dark text-sm mx-2">
                    <Icon icon={icondata::BsBag} width="16px" height="16px" {..} class="translate-y-px"/>
                    <p class="text-text-dark font-inter bg-base-dark mx-3 -translate-y-px" node_ref=shop_count>"0"</p>
                </a>

                <button on:click=toggle_active class="text-sm text-overlay-dark-200 mx-2 md:hidden">
                    <Icon icon=active_icon width="24px" height="24px"/>
                </button>
            </div>
        </nav>
    }
}
