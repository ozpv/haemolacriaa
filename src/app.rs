use futures::{channel::mpsc, Stream};
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, ProtectedRoute, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

use crate::{
    components::{buttons::ReturnButton, footer::Footer, nav::Nav},
    //pages::shop::products,
};
//use crate::components::forms::{logged_in, Login, LoginForm};
//use crate::pages::admin::Admin;
use crate::pages::home;
use crate::pages::releases::Releases;
//use crate::pages::shop;

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

fn watch_value() -> impl Stream<Item = ()> {
    #[allow(unused)]
    let (mut tx, rx) = mpsc::channel(0);

    #[cfg(feature = "ssr")]
    {
        use crate::lazy::UPDATE_ITEMS;

        tokio::spawn(async move {
            loop {
                if UPDATE_ITEMS.1.lock().await.changed().await.is_ok() {
                    let _ = tx.try_send(());
                    tracing::info!("Received update from `UPDATE_ITEMS`, regenerating /shop");
                }
            }
        });
    }

    rx
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/haemolacriaa.css"/>
        <Title text="haemolacriaa"/>

        <Router>
            <FlatRoutes fallback=NotFound>
                <Route path=path!("") view=home::Home />
                <Route path=path!("/releases/:name") view=Releases ssr=SsrMode::Async/>
                <Route path=path!("/shop") view=move || view! { <Todo message="no items found" /> }/>

                /* shop
                    <Route
                        path=path!("/shop")
                        view=shop::home::Home
                        ssr=SsrMode::Static(
                            StaticRoute::new().regenerate(|_| watch_value())
                        )
                    />
                    <Route
                        path=path!("/shop/:name")
                        view=products::Product
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route path=path!("/bag") view=shop::bag::Bag ssr=SsrMode::PartiallyBlocked />
                */
            </FlatRoutes>
            <Footer/>
        </Router>
    }
}

#[component]
pub fn Todo(message: &'static str) -> impl IntoView {
    view! {
        <Nav />
        <main class="bg-black min-h-screen">
            <h1 class="text-text-dark text-center pt-10 pb-7 text-2xl font-inter">{message}</h1>
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

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Nav/>
        <div class="bg-black min-h-screen">
            <h1 class="text-text-dark text-center pt-10 pb-7 text-2xl font-sans">"page not found"</h1>
            <ReturnButton href="/">"return home"</ReturnButton>
        </div>
    }
}
