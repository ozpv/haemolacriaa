use leptos::prelude::*;
use leptos_icons::Icon;

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
            <div class="bg-mantle-dark shadow md:rounded-lg hover:ease-in hover:duration-200 md:p-3 hover:bg-surface-dark">
                <img src=image class="h-auto max-w-full bg-transparent text-text-dark md:rounded-lg" alt=format!("{name} product image")/>
                <p class="text-text-dark text-lg font-inter pl-2 pt-3">{name}</p>
                <p class="text-subtext-dark font-inter pl-2 py-3">{price}</p>
            </div>
        </a>
    }
}

#[component]
pub fn SizeChartModal() -> impl IntoView {
    view! {
        <div tabindex="-1" class="fixed bg-crust-dark bg-opacity-80 max-h-full w-full h-full top-0 left-0 z-10 overflow-x-hidden overflow-y-auto p-4 md:inset-0" id="size-chart">
            <div class="relative w-full max-w-lg max-h-full">
                <div class="relative bg-base-dark rounded-lg w-full">
                    <div class="flex items-center justify-between border-b border-surface-dark rounded-t py-2">
                        <h3 class="text-text-dark font-sans text-xl py-3 px-8">
                            "size chart"
                        </h3>
                        <button class="text-overlay-dark-200 px-6">
                            <Icon icon={icondata::BsXLg} width="20px" height="20px" />
                        </button>
                    </div>
                    <div class="px-4 py-5">
                        <p class="text-text-dark font-inter">"Add the chart here"</p>
                    </div>
                    <div class="flex flex-row-reverse border-t border-surface-dark rounded-b px-4 py-3">
                        <button class="text-text-dark font-inter bg-surface-dark py-2 px-4 rounded hover:bg-surface-dark-100 hover:text-blue-dark">
                            "return"
                        </button>
                    </div>
                </div>
            </div>
        </div>

    }
}
