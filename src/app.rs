use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::pages::home::Home;

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

macro_rules! todo_page {
    () => {{
        multi_view!(Nav, Todo, Footer)
    }};
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
        <Title text="haemolacriaa"/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=multi_view!(Nav, Home, Footer) ssr=SsrMode::InOrder/>
                    <Route path="/blog" view=todo_page!() ssr=SsrMode::InOrder/>
                    <Route path="/*any" view=multi_view!(Nav, NotFound, Footer) ssr=SsrMode::InOrder/>
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

/// 404 Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="bg-gray-900 min-h-screen">
            <h1 class="text-white">"404 Not Found"</h1>
        </div>
    }
}
