use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use super::{item::List, nav::Nav};
use crate::components::buttons::ReturnButton;

#[component]
pub fn Home() -> impl IntoView {
    let on_click = move |_| {
        #[cfg(not(feature = "ssr"))]
        use super::storage::{get_storage, Bag};

        #[cfg(not(feature = "ssr"))]
        let product = super::item::Product::new("another product".to_string(), 32.0);

        #[cfg(not(feature = "ssr"))]
        let _ = Bag::try_add_bag_item(get_storage(), product);
    };

    view! {
        <Nav/>
        <button on:click=on_click>
            "Add item"
        </button>
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
