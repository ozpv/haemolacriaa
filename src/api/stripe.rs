use leptos::prelude::*;
use std::sync::{OnceLock, RwLock};

#[cfg(feature = "ssr")]
use crate::util::err;

use crate::types::product::{Product, Size};
use crate::util::Result;

static PRODUCTS: OnceLock<RwLock<Vec<Product>>> = OnceLock::new();

#[server(RegenItemsPage, "/api", "Url", endpoint = "regen_items_page")]
pub async fn regen_items_page() -> Result<()> {
    use crate::lazy::UPDATE_ITEMS;

    // Test
    PRODUCTS
        .get_or_init(|| RwLock::new(vec![Product::new("some product", 10000, Size::S)]))
        .write()
        .map_err(|_| ServerFnError::new("Failed to write to products"))?
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

    let products = PRODUCTS
        .get()
        .map(|inner| inner.read());

    if let Some(res) = products {
        let res = res
            .map_err(|_| ServerFnError::new("Failed to read products"))?
            .clone();

        Ok(Some(res))
    } else {
        Ok(None)
    }
}
