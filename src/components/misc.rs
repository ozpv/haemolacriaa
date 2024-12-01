use leptos::prelude::*;

#[component]
pub fn Divider(#[prop(optional)] class: Option<&'static str>) -> impl IntoView {
    view! {
        <hr class=format!("my-[30px] border-gray-800 mx-auto {}", class.unwrap_or("")) />
    }
}
