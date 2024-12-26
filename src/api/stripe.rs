use leptos::prelude::*;

use crate::types::product::Product;
#[cfg(feature = "ssr")]
use crate::util::err;
use crate::util::Result;

#[server(TryGetItemFromStripe, "/api", "GetJson")]
pub async fn try_get_item_from_stripe() -> Result<Vec<Product>> {
    err!("Not yet implemented")
}
