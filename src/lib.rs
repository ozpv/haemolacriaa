#![warn(clippy::pedantic)]
#![allow(clippy::must_use_candidate)]

//pub mod api;
pub mod app;
pub mod components;
pub mod config;
pub mod error;
pub mod pages;
pub mod types;

pub mod util {
    pub type Result<T, E = server_fn::ServerFnError> = std::result::Result<T, E>;

    /// err(REASON) <- to abstract and hide real error info
    /// err(REASON, STATUSCODE) <- set the STATUSCODE of the response
    // stuff is used but it says that it isn't
    #[allow(unused_macros)]
    macro_rules! err {
        ($s:tt) => {
            Err(server_fn::ServerFnError::new($s))
        };

        ($s:tt, $c:expr) => {
            leptos::expect_context::<ResponseOptions>().set_status($c);
            Err(server_fn::ServerFnError::new($s))
        };
    }

    #[allow(unused_imports)]
    pub(crate) use err;
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        //pub mod api;
        pub mod pool;

        pub mod lazy {
            use once_cell::sync::Lazy;

            pub static JWT_SECRET: Lazy<String> = Lazy::new(||
                std::env::var("JWT_SECRET").expect("Failed to parse jwt secret")
            );
        }
    } else if #[cfg(feature = "hydrate")] {
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn hydrate() {
            use crate::app::*;
            console_error_panic_hook::set_once();
            leptos::mount::hydrate_body(App);
        }
    }
}
