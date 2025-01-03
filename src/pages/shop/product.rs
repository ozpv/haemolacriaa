use leptos::{ev::MouseEvent, prelude::*};
use leptos_icons::Icon;
use leptos_router::{hooks::use_params, params::Params};
use std::sync::Arc;

use super::nav::Nav;

#[component]
pub fn Card(
    image: String,
    name: String,
    price: i64,
    #[prop(default = false)] in_stock: bool,
) -> impl IntoView {
    let redirect = if in_stock {
        Some(format!("/shop/{}", name.replace(' ', "-")))
    } else {
        None
    };

    let price = format!("${price}");

    view! {
        <a href=redirect class="content-none">
            <div class="bg-mantle-dark shadow md:rounded-lg hover:ease-in hover:duration-200 md:p-3 hover:bg-surface-dark">
                <img src=image class="h-auto max-w-full bg-transparent text-text-dark md:rounded-lg" alt=format!("{name} product image")/>
                <p class="text-text-dark text-md text-center font-sans pt-6">{name}</p>
                <p class="text-subtext-dark text-md text-center font-inter py-3">{price}</p>
            </div>
        </a>
    }
}

#[component]
fn SizeChartModal(
    toggle_status: Arc<impl FnMut(MouseEvent) + Copy + 'static>,
    status: impl FnMut() -> bool + Send + 'static,
) -> impl IntoView {
    view! {
        <div
            tabindex="-1"
            class="fixed bg-crust-dark bg-opacity-80 max-h-full w-full h-full top-0 left-0 z-10 overflow-x-hidden overflow-y-auto p-4 md:inset-0"
            id="size-chart"
            class:hidden=status
        >
            <div class="m-auto w-full max-w-lg max-h-full">
                <div class="relative bg-base-dark rounded-lg w-full">
                    <div class="flex items-center justify-between border-b border-surface-dark rounded-t py-2">
                        <h3 class="text-text-dark font-sans text-xl py-3 px-8">
                            "size chart"
                        </h3>
                        <button on:click=*Arc::clone(&toggle_status) class="text-overlay-dark-200 px-6">
                            <Icon icon={icondata::BsXLg} width="20px" height="20px" />
                        </button>
                    </div>
                    <div class="px-4 py-5">
                        <p class="text-text-dark font-inter">"Add the chart here"</p>
                    </div>
                    <div class="flex flex-row-reverse border-t border-surface-dark rounded-b px-4 py-3">
                        <button on:click=*toggle_status class="text-text-dark font-inter bg-surface-dark py-2 px-4 rounded hover:bg-surface-dark-100 hover:text-blue-dark">
                            "return"
                        </button>
                    </div>
                </div>
            </div>
        </div>

    }
}

#[component]
fn Breadcrumb(path: impl FnOnce() -> String + Send) -> impl IntoView {
    let path = path();
    let haref = format!("/shop/{path}");

    view! {
        <nav class="">
            <ol class="items-center">
                <li class="text-text-dark">
                    <a href="/shop" class="font-inter hover:text-sapphire-dark">"shop"</a>
                    <Icon icon={icondata::RiArrowRightSArrowsLine}/>
                </li>
                <li class="text-text-dark">
                    <a href=haref class="font-inter hover:text-sapphire-dark">{path}</a>
                </li>
            </ol>
        </nav>
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
            .replace('-', " ")
            .trim()
            .to_string()
    };

    let add_to_bag = view! {
        <button class="flex items-center justify-center p-0.5 shadow-lg shadow-mantle-dark rounded-lg group bg-gradient-to-br from-yellow-dark to-blue-dark">
            <span
                class="text-text-dark text-md text-center font-inter py-6 w-80 transition-all ease-in duration-75 bg-base-dark rounded-md hover:bg-opacity-0 hover:scale-105 hover:text-base-dark"
            >
                "add to bag"
            </span>
        </button>
    };

    let activity = RwSignal::new(false);
    let toggle_modal = move |_| activity.update(|status| *status = !*status);
    let status = move || !activity.get();

    view! {
        <SizeChartModal toggle_status=toggle_modal.into() status=status />
        <Nav/>
        <main class="main">
            <Breadcrumb path=move || id() />
            <div class="m-auto">
                <h3 class="text-text-dark text-lg font-sans">"Product Title"</h3>
                <p class="text-text-dark font-inter">"Product description"</p>
                <button
                    on:click=toggle_modal
                    class="text-text-dark font-inter"
                >
                    "Size Chart"
                </button>
                {add_to_bag}
            </div>
        </main>
    }
}
