use leptos::prelude::*;

#[component]
pub fn Card(
    image: String,
    name: String,
    price: i64,
    #[prop(default = false)] in_stock: bool,
) -> impl IntoView {
    let redirect = format!("/shop/{}", if in_stock { &name } else { "" });

    let price = format!("${price}");

    view! {
        <a href=redirect class="content-none">
            <div class="bg-mantle-dark shadow rounded hover:ease-in hover:duration-200 md:p-3 hover:bg-surface-dark">
                <img src=image width="300px" height="300px" class="text-text-dark" alt=format!("{name} product image")/>
                <p class="text-text-dark text-lg font-inter pl-2 pt-3">{name}</p>
                <p class="text-subtext-dark font-inter pl-2 py-3">{price}</p>
            </div>
        </a>
    }
}
