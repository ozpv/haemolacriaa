use leptos::prelude::*;
use std::sync::{LazyLock, RwLock};

#[cfg(feature = "ssr")]
use crate::util::err;

use crate::types::product::Product;
use crate::util::Result;

static ITEMS: LazyLock<RwLock<Vec<Product>>> =
    LazyLock::new(|| RwLock::new(vec![Product::new("some product", 10000)]));

#[server(RegenItemsPage, "/api", "Url", endpoint = "regen_items_page")]
pub async fn regen_items_page() -> Result<()> {
    use crate::lazy::UPDATE_ITEMS;

    // Test
    ITEMS
        .write()
        .unwrap()
        .push(Product::new("another product", 10000));

    UPDATE_ITEMS.0.lock().await.send(()).map_err(|e| {
        tracing::info!("Failed to regen /shop: {e}");
        ServerFnError::new("Failed to regen /shop")
    })
}

pub async fn get_items_from_stripe() -> Result<Vec<Product>> {
    #[cfg(feature = "ssr")]
    tracing::info!("Fetching items from stripe");
    let items = ITEMS
        .read()
        .map_err(|_| ServerFnError::new("Failed to read from RwLock"))?
        .clone();
    Ok(items)
}
