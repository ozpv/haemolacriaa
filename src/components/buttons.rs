use leptos::prelude::*;

#[component]
pub fn LinkButton(
    class: &'static str,
    href: String,
    id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="flex items-center justify-center p-px mt-2.5 rounded-lg group bg-gradient-to-br from-yellow-dark to-blue-dark" href=href id=id>
            <span class=format!("{} {}",
                "flex justify-center transition-all ease-in duration-75 bg-base-dark rounded-md group-hover:bg-opacity-0",
                class
            )>
                { children() }
            </span>
        </a>
    }
}
