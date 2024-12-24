use leptos::prelude::*;

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
    view! {
        <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
            {(0..10)
                .into_iter()
                .map(|i| view! {
                    <Card image_path="stay.webp".to_string() product_name=i.to_string() price=100.0 + (i as f64) in_stock=true />
                })
                .collect_view()
            }
        </div>
    }
}
