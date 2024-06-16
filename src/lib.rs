#![feature(const_trait_impl, effects)]

pub mod app;
pub mod components;
pub mod config;
pub mod error;
#[cfg(feature = "ssr")]
pub mod fileserv;
pub mod pages;
pub mod types;
#[cfg(feature = "ssr")]
pub mod song_db;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
