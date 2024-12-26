use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use super::{item, nav::Nav};
use crate::components::buttons::ReturnButton;
#[cfg(feature = "hydrate")]
use crate::types::product;

#[component]
pub fn Home() -> impl IntoView {
    let add_item = move |_| {
        #[cfg(feature = "hydrate")]
        use super::storage::{get_storage, Bag};
        #[cfg(feature = "hydrate")]
        let product = product::Product::new("another product".to_string(), 32);
        #[cfg(feature = "hydrate")]
        Bag::try_add_to_bag(get_storage(), product).unwrap();
    };

    let total_element = NodeRef::new();

    let total_bag = move |_| {
        #[cfg(feature = "hydrate")]
        use super::storage::{get_storage, Bag};
        #[cfg(feature = "hydrate")]
        use web_sys::HtmlButtonElement;
        #[cfg(feature = "hydrate")]
        let total = Bag::try_total_bag(get_storage().as_ref()).unwrap();
        #[cfg(feature = "hydrate")]
        let total_element: HtmlButtonElement = total_element.get().unwrap();
        #[cfg(feature = "hydrate")]
        Dom::set_inner_html(&total_element, &format!("Bag total: {total}"))
    };

    view! {
        <Nav/>
        <button on:click=add_item>
            "Add item"
        </button>
        <button on:click=total_bag node_ref=total_element>
            "Bag total: 0"
        </button>
        <main class="main">
            <h1 class="text-text-dark text-5xl text-center font-sans py-5">"shop"</h1>
            <item::List />
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
