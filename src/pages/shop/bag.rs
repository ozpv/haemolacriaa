use leptos::prelude::*;

use crate::components::buttons::ReturnButton;
use crate::components::nav::ShopNav;

#[component]
pub fn Bag() -> impl IntoView {
    view! {
        <ShopNav />
        <main class="main">
            <h1 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"your bag is empty"</h1>
            <ReturnButton href="/shop" external=true>"continue shopping"</ReturnButton>
        </main>
    }
}
