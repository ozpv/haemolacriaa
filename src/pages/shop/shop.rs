use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::{hooks::use_params, params::Params};

use super::item::List;
use crate::components::buttons::ReturnButton;
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
                                    <a href=item.path class="text-text-dark font-inter p-2.5 hover:bg-base-dark md:hover:text-sapphire-dark md:hover:bg-transparent">
                                        {item.name}
                                    </a>
                                </li>
                            })
                            .collect_view()
                        }
                    </ul>
                </div>

                <a href="/bag" class="flex flew-row text-text-dark text-sm mx-2">
                    <Icon icon={icondata::BsBag} width="16px" height="16px" {..} class="translate-y-px"/>
                    <p class="text-text-dark font-inter bg-base-dark mx-3 -translate-y-px">"0"</p>
                </a>

                <button on:click=toggle_active class="text-sm text-overlay-dark-200 mx-2 md:hidden">
                    <Icon icon=active_icon width="24px" height="24px"/>
                </button>
            </div>
        </nav>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Nav/>
        <main class="main">
            <h1 class="text-text-dark text-5xl text-center font-sans py-5">"shop"</h1>
            <List />
        </main>
    }
}

#[component]
pub fn Bag() -> impl IntoView {
    view! {
        <Nav />
        <main class="main">
            <h1 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"your bag is empty"</h1>
            <ReturnButton body="continue shopping" href="/shop" />
        </main>
    }
}

#[derive(Params, PartialEq)]
struct ProductParams {
    name: Option<String>,
}

#[component]
pub fn Product() -> impl IntoView {
    let params = use_params::<ProductParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.name.clone())
            .unwrap_or("Invalid ID".to_string())
    };

    view! {
        <Nav/>
        <main class="main">
            <p>"This is a product page"</p>
            <p>"Your ID is: "{id}</p>
        </main>
    }
}
