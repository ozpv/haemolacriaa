use leptos::prelude::*;

use crate::components::{buttons::ReturnButton, nav};

#[component]
pub fn ErrorPage() -> impl IntoView {
    view! {
        <nav::Nav/>
        <div class="bg-base-dark min-h-screen">
            <h2 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"page not found"</h2>
            <ReturnButton body="return home" href="/" />
        </div>
    }
}
