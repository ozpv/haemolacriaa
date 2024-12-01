use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{path, components::{Redirect, Route, Router, Routes}, nested_router::Outlet, SsrMode};

// use crate::components::forms::{logged_in, LoginForm};
use crate::components::nav::Nav;
use crate::components::footer::Footer;
// use crate::error::{AppError, ErrorPage};
// use crate::pages::admin::Admin;
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

    // let logged_in = Action::new(|(): &()| async { logged_in().await });

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
                <Routes fallback=|| "Not Found".into_view()>
                    <Route path=path!("/") view=Home ssr=SsrMode::InOrder/>
                    <Route path=path!("/shop") view=Shop ssr=SsrMode::InOrder/>

                    // login page
                    //<Route path=path!("/login") view=move || view! {
                    //    <Suspense>
                    //        // if user is logged in, redirect to /admin
                    //        { logged_in.dispatch(()); }
                    //        <Show when=move || logged_in.value().get().is_some_and(|res| res.is_ok())>
                    //            <Redirect path="/admin"/>
                    //        </Show>
                    //    </Suspense>
                    //    // otherwise show the login panel
                    //    <Outlet/>
                    //}>
                    //    <Route path="" view=LoginForm/>
                    //</Route>

                    //// admin panel
                    //<Route path=path!("/admin") view=move || view! {
                    //    <Suspense>
                    //        // check if logged in, if not redirect to /login
                    //        <Show when=move || logged_in.value().get().is_none_or(|res| res.is_err())>
                    //            <Redirect path="/login"/>
                    //        </Show>
                    //    </Suspense>
                    //    // else (if logged in) show the children
                    //    <Outlet/>
                    //}>
                    //    <Route path=path!("") view=Admin/>
                    //</Route>
                </Routes>
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
