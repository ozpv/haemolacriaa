#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]

pub mod api;
pub mod app;
pub mod components;
pub mod config;
pub mod pages;
pub mod types;

#[cfg(feature = "ssr")]
pub mod pool;
#[cfg(feature = "ssr")]
pub mod util;
#[cfg(feature = "ssr")]
pub mod lazy {
    use crate::types::product::Product;
    use std::sync::LazyLock;
    use tokio::sync::{
        watch::{Receiver, Sender},
        Mutex,
    };

    pub static JWT_SECRET: LazyLock<String> =
        LazyLock::new(|| std::env::var("JWT_SECRET").expect("Failed to parse jwt secret"));

    pub static UPDATE_ITEMS: LazyLock<(Mutex<Sender<()>>, Mutex<Receiver<()>>)> =
        LazyLock::new(|| {
            let (tx, rx) = tokio::sync::watch::channel(());
            (Mutex::new(tx), Mutex::new(rx))
        });
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
