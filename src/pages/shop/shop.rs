use super::nav::Nav;
use super::product::{Card, Product};
use crate::components::buttons::ReturnButton;
#[allow(unused)]
use crate::types;
use leptos::prelude::*;

#[component]
fn OutOfStock() -> impl IntoView {
    view! {
        <p class="text-text-dark">"No items found"</p>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let add_item = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, bag};
            let product =
                types::product::Product::new("another-product", 32, types::product::Size::XS);
            bag::try_add_to_bag(get_storage().as_ref(), product).unwrap();
        }
    };

    let total_element = NodeRef::new();

    let total_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, bag};
            use web_sys::HtmlButtonElement;
            let total = bag::try_total_bag(get_storage().as_ref()).unwrap();
            let total_element: HtmlButtonElement = total_element.get().unwrap();
            Dom::set_inner_html(&total_element, &format!("Bag total: {total}"))
        }
    };

    let sync_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{get_storage, bag};
            bag::try_sync_bag_count(get_storage().as_ref()).unwrap();
        }
    };

    let items_resource = OnceResource::new_blocking(crate::api::stripe::get_products());

    let items_view = move || {
        Suspend::new(async move {
            items_resource.await.map(|items| {
                items
                    .map_or(OutOfStock.into_any(), |products| products
                        .into_iter()
                        .map(|product| {
                            view! {
                                <Card
                                    image="stay.webp".to_string()
                                    name=product.get_name()
                                    price=product.get_price()
                                    in_stock=true
                                />
                            }
                        })
                        .collect_view()
                        .into_any()
                    )
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
                    <div class="grid grid-cols-2 gap-5 p-4 md:grid-cols-4">
                        {items_view}
                    </div>
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
