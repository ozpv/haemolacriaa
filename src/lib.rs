pub mod app;
pub mod components;
pub mod config;
pub mod error;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod jwt;
pub mod pages;
pub mod song_db;
#[cfg(feature = "ssr")]
pub mod state;
pub mod types;
#[cfg(feature = "ssr")]
pub mod upload;

#[cfg(feature = "ssr")]
pub mod lazy {
    use crate::jwt::Keys;
    use once_cell::sync::Lazy;

    pub static KEYS: Lazy<Keys> = Lazy::new(|| {
        let secret = std::env::var("JWT_SECRET").expect("Failed to parse jwt secret!");
        Keys::new(secret.as_bytes())
    });

    pub static IMAGE_UPLOAD_DIR: Lazy<String> = Lazy::new(|| {
        let dir = std::env::var("IMG_UPLOAD_DIR").expect("Failed to parse image upload dir!");
        String::from(dir)
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
