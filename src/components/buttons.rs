use leptos::{html, prelude::*};
use leptos_icons::Icon;

#[component]
pub fn LinkButton(href: String, children: Children) -> impl IntoView {
    view! {
        <a class="flex items-center justify-center p-px mt-4 shadow-lg shadow-mantle-dark rounded-lg group bg-gradient-to-br from-yellow-dark to-lavender-dark" href=href>
            <span
                class="flex justify-center transition-all ease-in duration-75 bg-black rounded-md group-hover:bg-opacity-0 text-text-dark text-md font-inter py-6 w-80 hover:scale-105 hover:text-black"
            >
                { children() }
            </span>
        </a>
    }
}

#[component]
pub fn ReturnButton(
    href: &'static str,
    #[prop(default = false)] external: bool,
    children: Children,
) -> impl IntoView {
    let button = html::a()
        .href(href)
        .class("flex justify-center bg-surface-dark rounded-full text-text-dark pr-6 pl-8 py-3 hover:bg-surface-dark-100 hover:text-blue-dark")
        .rel(external.then_some("external"))
        .child((html::p()
                .class("text-center font-inter pr-3")
                .child(children()),
            view! {
                // Would use builder syntax but it's more of a headache 
                <Icon icon={icondata::BsArrowRight} width="20" height="20" {..} class="translate-y-0.5" />
            },
        ));

    view! {
        <div class="flex justify-center">
            {button}
        </div>
    }
}
