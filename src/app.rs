use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, ProtectedRoute, Route, Router},
    SsrMode, StaticSegment,
};

use crate::components::footer::Footer;
use crate::components::forms::{logged_in, Login, LoginForm};
use crate::components::nav::Nav;
use crate::error::{AppError, ErrorPage};
use crate::pages::admin::Admin;
use crate::pages::home::Home;
use crate::pages::shop::Shop;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let logged_in = Resource::new(move || (), |_| logged_in());

    //let fallback = || {
    //    let mut outside_errors = Errors::default();
    //    outside_errors.insert_with_default_key(AppError::NotFound);
    //    view! {
    //        <ErrorPage outside_errors/>
    //    }
    //};

    view! {
        <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
        <Title text="haemolacriaa"/>

        <Router>
            <Nav/>
            <main>
                <FlatRoutes fallback=move || view! { <p>"404 Error"</p> }>
                    <Route path=StaticSegment("/") view=Home />
                    <Route path=StaticSegment("/shop") view=Shop />
                    <ProtectedRoute
                        path=StaticSegment("/login")
                        view=LoginForm
                        condition=move || Some(logged_in.get().is_none_or(|res| res.is_err()))
                        redirect_path=|| "/admin"
                    />
                    <ProtectedRoute
                        path=StaticSegment("/admin")
                        view=Admin
                        condition=move || Some(logged_in.get().is_some_and(|res| res.is_ok()))
                        redirect_path=|| "/login"
                        ssr=SsrMode::InOrder
                    />
                </FlatRoutes>
            </main>
            <Footer/>
        </Router>
    }
}

/// Todo
#[component]
fn Todo() -> impl IntoView {
    view! {
        <Nav/>
        <div class="bg-gray-900 min-h-screen">
            <h1 class="text-white">"Work in progress"</h1>
        </div>
    }
}
