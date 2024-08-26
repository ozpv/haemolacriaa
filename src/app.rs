use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::error::{AppError, ErrorPage};
use crate::pages::home::Home;
use crate::pages::releases::Releases;
use crate::pages::shop::Shop;

/// View multiple components in Router, pass routes in succession
macro_rules! multi_view {
    ($component:ty) => {
        {
            move || view!(<$component/>)
        }
    };

    ($component:ty, $($components:ty),+) => {
        {
            move || view!(<$component/> { multi_view!($($components),+) })
        }
    };
}
/*
macro_rules! todo_page {
    () => {{
        multi_view!(Nav, Todo)
    }};
}
*/

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
        <Title text="haemolacriaa"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorPage outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=multi_view!(Nav, Home, Footer) ssr=SsrMode::InOrder/>
                    <Route path="/shop" view=multi_view!(Nav, Shop, Footer) ssr=SsrMode::InOrder/>
                    <Route path="/releases/:name" view=multi_view!(Nav, Releases, Footer) ssr=SsrMode::InOrder/>
                </Routes>
            </main>
        </Router>
    }
}

/// Todo
#[component]
fn Todo() -> impl IntoView {
    view! {
        <div class="bg-gray-900 min-h-screen">
            <h1 class="text-white">"Work in progress"</h1>
        </div>
    }
}
