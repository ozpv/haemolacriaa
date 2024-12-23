use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, ProtectedRoute, Route, Router},
    path, SsrMode,
};

use crate::components::footer::Footer;
//use crate::components::forms::{logged_in, Login, LoginForm};
use crate::components::nav;
use crate::error::ErrorPage;
//use crate::pages::admin::Admin;
use crate::pages::home;
use crate::pages::shop::shop;

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

    //let logged_in = Resource::new(move || (), |_| logged_in());

    view! {
            <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
            <Title text="haemolacriaa"/>

            <Router>
                <FlatRoutes fallback=ErrorPage>
                    <Route path=path!("") view=home::Home />
                    <Route path=path!("/shop") view=shop::Home />
                    <Route path=path!("/shop/:name") view=shop::Product />
                    <Route path=path!("/bag") view=shop::Bag />
    /*
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
    */
                </FlatRoutes>
                <Footer/>
            </Router>
        }
}

#[component]
pub fn Todo() -> impl IntoView {
    view! {
        <nav::Nav />
        <main class="bg-base-dark min-h-screen">
            <h2 class="text-text-dark text-center pt-10 pb-7 text-2xl font-inter">"Coming soon"</h2>
            <div class="flex justify-center">
                <a href="/" class="flex justify-center bg-surface-dark rounded-full text-text-dark pr-6 pl-8 py-3 hover:bg-surface-dark-100 hover:text-blue-dark">
                    <p class="text-center font-inter pr-3">
                        "Return home"
                    </p>
                    <Icon icon={icondata::BsArrowRight} width="20" height="20" {..} class="translate-y-0.5" />
                </a>
            </div>
        </main>
    }
}
