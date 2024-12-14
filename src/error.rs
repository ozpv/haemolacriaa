use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn ErrorPage() -> impl IntoView {
    view! {
        <div class="bg-base-dark min-h-screen">
            <h2 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"Page Not Found"</h2>
            <div class="flex justify-center">
                <a class="flex justify-center bg-surface-dark rounded-full text-text-dark pr-6 pl-8 py-3 hover:bg-surface-dark-100 hover:text-blue-dark" href="/">
                    <p class="text-center font-sans pr-3">
                        "Return home"
                    </p>
                    <Icon icon={icondata::BsArrowRight} width="20" height="20" {..} class="translate-y-0.5" />
                </a>
            </div>
        </div>
    }
}
