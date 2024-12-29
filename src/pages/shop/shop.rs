use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

use super::nav::Nav;
use crate::components::buttons::ReturnButton;
use crate::pages::shop::product::Card;
use crate::types::product;

#[component]
pub fn Home() -> impl IntoView {
    let add_item = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, Bag};
            let product = product::Product::new("another product", 32, product::Size::XS);
            Bag::try_add_to_bag(get_storage().as_ref(), product).unwrap();
        }
    };

    let total_element = NodeRef::new();

    let total_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, Bag};
            use web_sys::HtmlButtonElement;
            let total = Bag::try_total_bag(get_storage().as_ref()).unwrap();
            let total_element: HtmlButtonElement = total_element.get().unwrap();
            Dom::set_inner_html(&total_element, &format!("Bag total: {total}"))
        }
    };

    let sync_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, Bag};
            Bag::try_sync_bag_count(get_storage().as_ref()).unwrap();
        }
    };

    let items_resource = OnceResource::new_blocking(crate::api::stripe::get_products());

    let items_view = move || {
        Suspend::new(async move {
            items_resource.await.map(|items| {
                items
                    .into_iter()
                    .map(|item| {
                        view! {
                            <Card
                                image="stay.webp".to_string()
                                name=item.get_name()
                                price=item.get_price()
                                in_stock=true
                            />
                        }
                    })
                    .collect_view()
            })
        })
    };

    view! {
        <Nav/>
        <main class="main">
            <button on:click=add_item>
                "Add item"
            </button>

            <button on:click=total_bag node_ref=total_element>
                "Bag total: 0"
            </button>

            <button on:click=sync_bag>
                "Sync bag count"
            </button>

            <h1 class="text-text-dark text-5xl text-center font-sans py-5">"shop"</h1>

            <Suspense fallback=move || view! {
                <p class="text-text-dark text-center font-inter">"Loading products..."</p>
            }>
                <ErrorBoundary fallback=move |_| view! {
                    <p class="text-text-dark text-center font-inter">"No products found"</p>
                }>
                    {items_view}
                </ErrorBoundary>
            </Suspense>
        </main>
    }
}

#[component]
pub fn Bag() -> impl IntoView {
    view! {
        <Nav />
        <main class="main">
            <h1 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"your bag is empty"</h1>
            <ReturnButton body="continue shopping" href="/shop" external=true />
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
        <product::SizeChartModal />
        <Nav/>
        <main class="main">
            <p>"This is a product page"</p>
            <p>"Your ID is: "{id}</p>
        </main>
    }
}
