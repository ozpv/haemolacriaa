use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::footer::Footer;
use crate::components::forms::{logged_in, LoginForm};
use crate::components::nav::Nav;
use crate::error::{AppError, ErrorPage};
use crate::pages::admin::Admin;
use crate::pages::home::Home;
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

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let logged_in = create_action(|_: &()| async { logged_in().await });

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

                    // login page
                    <Route path="/login" view=move || view! {
                        <Suspense>
                            // if user is logged in, redirect to /admin
                            { logged_in.dispatch(()); }
                            <Show when=move || logged_in.value().get().is_some_and(|res| res.is_ok())>
                                <Redirect path="/admin"/>
                            </Show>
                        </Suspense>
                        // otherwise show the login panel
                        <Outlet/>
                    }>
                        <Route path="" view=multi_view!(Nav, LoginForm)/>
                    </Route>

                    // admin panel
                    <Route path="/admin" view=move || view! {
                        <Suspense>
                            // check if logged in, if not redirect to /login
                            { logged_in.dispatch(()); }
                            <Show when=move || logged_in.value().get().is_none_or(|res| res.is_err())>
                                <Redirect path="/login"/>
                            </Show>
                        </Suspense>
                        // else (if logged in) show the children
                        <Outlet/>
                    }>
                        <Route path="" view=Admin/>
                    </Route>
                </Routes>
            </main>
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
