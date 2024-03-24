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
    // todo is to extend this macro and allow for props.
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
        <Title text="haemolacriaa"/>

        <Nav/>

        <Router>
            <main>
                <Routes>
                    <Route path="/" view=multi_view!(Home, Footer) ssr=SsrMode::InOrder/>
                    <Route path="/*any" view=multi_view!(<NotFound todo=true/>, Footer) ssr=SsrMode::InOrder/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 Not Found
#[component]
fn NotFound(#[prop(optional)] todo: bool) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <body class="bg-gray-900 min-h-screen">
            <h1 class="text-white">
                {
                    move || if todo { "Work in progress..." } else { "404 Not Found" }
                }
            </h1>
        </body>
    }
}
