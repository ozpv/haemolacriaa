pub mod app;
pub mod components;
pub mod config;
pub mod error;
pub mod pages;
pub mod song_db;
pub mod types;

pub mod util {
    pub type Result<T, E = leptos::ServerFnError> = std::result::Result<T, E>;

    // stuff is used but it says that it isn't
    #[allow(unused_macros)]
    macro_rules! err {
        ($s:tt) => {
            Err(leptos::ServerFnError::new($s))
        };
    }

    #[allow(unused_imports)]
    pub(crate) use err;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod fileserv;
        pub mod jwt;
        pub mod state;

        pub mod lazy {
            use crate::jwt::Keys;
            use once_cell::sync::Lazy;

            pub static KEYS: Lazy<Keys> = Lazy::new(|| {
                let secret = std::env::var("JWT_SECRET").expect("Failed to parse jwt secret!");
                Keys::new(secret.as_bytes())
            });
        }
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
