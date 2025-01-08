#[cfg(feature = "ssr")]
use http::StatusCode;
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
#[cfg(feature = "ssr")]
use std::sync::{OnceLock, RwLock};

use crate::types::product::Product;
#[cfg(feature = "ssr")]
use crate::types::product::Size;
#[cfg(feature = "ssr")]
use crate::util::err;
use crate::util::Result;

#[cfg(feature = "ssr")]
static PRODUCTS: OnceLock<RwLock<Vec<Product>>> = OnceLock::new();

#[server(RegenItemsPage, "/api", "Url", endpoint = "regen_items_page")]
pub async fn regen_items_page() -> Result<()> {
    use crate::lazy::UPDATE_ITEMS;

    let products =
        PRODUCTS.get_or_init(|| RwLock::new(vec![Product::new("some product", 10000, Size::S)]));

    if products.read().is_ok_and(|v| v.len() > 100) {
        expect_context::<ResponseOptions>().set_status(StatusCode::IM_A_TEAPOT);
        return Err(ServerFnError::new("Too may items inside of PRODUCTS"));
    }

    products
        .write()
        .map_err(|_| ServerFnError::new("Failed to get write lock on PRODUCTS"))?
        .push(Product::new("Hello, World!", 10000, Size::XS));

    UPDATE_ITEMS
        .0
        .lock()
        .await
        .send(())
        .map_err(|_| ServerFnError::new("Failed to regen /shop"))
}

#[server]
pub async fn get_products() -> Result<Option<Vec<Product>>> {
    #[cfg(feature = "ssr")]
    tracing::info!("Fetching items from stripe");

    let products = PRODUCTS.get().map(|inner| inner.read());

    if let Some(res) = products {
        let res = res
            .map_err(|_| ServerFnError::new("Failed to read products"))?
            .clone();

        Ok(Some(res))
    } else {
        Ok(None)
    }
}
