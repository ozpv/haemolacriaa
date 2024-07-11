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
#[cfg(feature = "ssr")]
pub mod app_state;
#[cfg(feature = "ssr")]
pub mod jwt;

#[cfg(feature = "ssr")] 
pub mod keys {
    use once_cell::sync::Lazy;
    use crate::jwt::Keys;

    pub static KEYS: Lazy<Keys> = Lazy::new(|| {
        let secret = std::env::var("JWT_SECRET").expect("Failed to parse jwt secret!");
        Keys::new(secret.as_bytes())
    });
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
