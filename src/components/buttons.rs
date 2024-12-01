use leptos::prelude::*;

#[component]
pub fn LinkButton(
    class: &'static str,
    href: String,
    id: String,
    children: Children,
) -> impl IntoView {
    view! {
        <a class="relative flex items-center justify-center p-0.5 mt-[10px] rounded-lg group bg-gradient-to-br from-yellow-950 to-blue-900" href=href id=id>
            <span class=format!("{} {}",
                    "relative flex justify-center transition-all ease-in duration-75 bg-gray-900 rounded-md group-hover:bg-opacity-0",
                    class
                )
            >
                { children() }
            </span>
        </a>
    }
}
