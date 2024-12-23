use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <p>"Shop nav"</p>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Nav/>
        <p>"Welcome home"</p>
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
        <p>"This is a product page"</p>
        <p>"Your ID is: "{id}</p>
    }
}
