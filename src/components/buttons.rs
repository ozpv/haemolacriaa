use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn LinkButton(
    class: &'static str,
    href: String,
    id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="flex items-center justify-center p-px mt-4 shadow-lg shadow-mantle-dark rounded-lg group bg-gradient-to-br from-yellow-dark to-blue-dark" href=href id=id>
            <span
                class=format!(
                    "flex justify-center transition-all ease-in duration-75 bg-base-dark rounded-md group-hover:bg-opacity-0 {}",
                    class
                )
            >
                { children() }
            </span>
        </a>
    }
}

#[component]
pub fn ReturnButton(body: &'static str, href: &'static str) -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <a href=href class="flex justify-center bg-surface-dark rounded-full text-text-dark pr-6 pl-8 py-3 hover:bg-surface-dark-100 hover:text-blue-dark">
                <p class="text-center font-inter pr-3">{body}</p>
                <Icon icon={icondata::BsArrowRight} width="20" height="20" {..} class="translate-y-0.5" />
            </a>
        </div>
    }
}
