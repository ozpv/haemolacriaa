use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen::prelude::wasm_bindgen]
#[cfg_attr(not(feature = "ssr"), derive(Deserialize, Serialize))]
#[derive(Clone)]
pub struct Product {
    name: String,
    price: f64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl Product {
    #[wasm_bindgen::prelude::wasm_bindgen(constructor)]
    pub fn new(name: String, price: f64) -> Self {
        Self { name, price }
    }

    pub fn get_price(&self) -> f64 {
        self.price
    }

    pub fn get_name(self) -> String {
        self.name
    }
}

#[component]
fn Card(
    image_path: String,
    product_name: String,
    price: f64,
    #[prop(default = false)] in_stock: bool,
) -> impl IntoView {
    let redirect = format!("/shop/{}", if in_stock { &product_name } else { "" });

    let price = format!("${price}");

    view! {
        <a href=redirect class="content-none">
            <div class="bg-mantle-dark shadow rounded hover:ease-in hover:duration-200 md:p-3 hover:bg-surface-dark">
                <img src=image_path width="300px" height="300px" class="text-text-dark" alt=format!("{product_name} product image")/>
                <p class="text-text-dark text-lg font-inter pl-2 pt-3">{product_name}</p>
                <p class="text-subtext-dark font-inter pl-2 py-3">{price}</p>
            </div>
        </a>
    }
}

#[component]
pub fn List() -> impl IntoView {
    let items = RwSignal::<Option<Vec<Product>>>::new(None);

    #[cfg(not(feature = "ssr"))]
    Effect::new(move || {
        use super::storage::{get_storage, Bag};

        let stored_items = Bag::try_get_bag_items(get_storage()).ok();
        items.set(stored_items);
    });

    // TODO: Get Items from localstorage or try the server and not from the bag
    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
            {move || {
                items.get().map_or(().into_any(), |items| {
                    items
                        .iter()
                        .map(|item| {
                            view! {
                                <Card
                                    image_path="stay.webp".to_string()
                                    product_name=item.clone().get_name()
                                    price=item.get_price()
                                    in_stock=true
                                />
                            }
                        })
                        .collect_view()
                        .into_any()
                })
            }}
        </div>
    }
}
