use super::products::Card;
use crate::components::nav::ShopNav;
#[allow(unused)]
use crate::types;
use leptos::prelude::*;

#[component]
fn OutOfStock() -> impl IntoView {
    view! {
        <p class="text-text-dark text-2xl text-center font-sans py-5 lg:text-3xl xl:text-4xl">
            "No products found"
        </p>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let add_item = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{bag, get_storage};
            let product =
                types::product::Product::new("another-product", 32, types::product::Size::XS);
            bag::try_add_to_bag(get_storage().as_ref(), product).unwrap();
        }
    };

    let total_element = NodeRef::new();

    let total_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{bag, get_storage};
            use web_sys::HtmlButtonElement;
            let total = bag::try_total_bag(get_storage().as_ref()).unwrap();
            let total_element: HtmlButtonElement = total_element.get().unwrap();
            Dom::set_inner_html(&total_element, &format!("Bag total: {total}"))
        }
    };

    let sync_bag = move |_| {
        #[cfg(feature = "hydrate")]
        {
            use super::storage::{bag, get_storage};
            bag::try_sync_bag_count(get_storage().as_ref()).unwrap();
        }
    };

    let items_resource = OnceResource::new_blocking(crate::api::stripe::get_products());

    let items_view = move || {
        Suspend::new(async move {
            items_resource.await.map(|items| {
                items.map_or(OutOfStock.into_any(), |products| view! {
                    <h1 class="text-text-dark text-3xl text-center font-sans py-5 lg:text-4xl xl:text-5xl">"shop"</h1>
                    <div class="grid grid-cols-1 gap-5 p-4 xl:mx-[10%] lg:grid-cols-3">
                        {
                            products
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
                        }
                    </div>
                }
                .into_any())
            })
        })
    };

    view! {
        <ShopNav />
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

            <Suspense fallback=move || view! {
                <p class="text-text-dark text-center font-inter">"Loading products..."</p>
            }>
                <ErrorBoundary fallback=move |_| OutOfStock>
                    {items_view}
                </ErrorBoundary>
            </Suspense>
        </main>
    }
}
