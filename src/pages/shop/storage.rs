use js_sys::Error;
use leptos::{leptos_dom::helpers, tachys::html::event::Event};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Storage;

use crate::types::product::Product;

#[inline]
pub fn get_storage() -> Option<Storage> {
    helpers::window().local_storage().ok()?
}

pub fn try_dispatch_storage_event() -> Result<(), JsValue> {
    let window = helpers::window();
    let event = Event::new("storage")?;

    let _ = window.dispatch_event(&event)?;

    Ok(())
}

#[inline]
fn js_exception<'a>(s: &'a str) -> JsValue {
    Error::new(s).into()
}

pub struct Bag;

impl Bag {
    /// Gets the value of `bag_count` from `storage`
    /// Deletes the invalid value and returns an error if `bag_count` doesn't exist
    pub fn try_get_bag_count(storage: Option<&Storage>) -> Result<usize, JsValue> {
        let storage = storage.ok_or_else(|| js_exception("Invalid storage object"))?;

        storage
            .get_item("bag_count")?
            .as_deref()
            .map(<str>::parse::<usize>)
            .ok_or_else(|| js_exception("Failed to find bag_count"))?
            .map_err(|_| {
                let _ = storage.delete("bag_count");
                js_exception("Failed to parse bag_count, deleted invalid value")
            })
    }

    /// Gets the value of `bag_count` from `storage`
    /// If the `bag_count` doesn't exist, return 0
    pub fn get_bag_count_or_default(storage: Option<&Storage>) -> usize {
        Self::try_get_bag_count(storage).unwrap_or(0)
    }

    /// "syncs" `bag_count` by setting it to the sum of values in each `bag` from `storage`
    /// Returns an error if `bag_count` doesn't exist, or dispatch_event fails
    pub fn try_sync_bag_count(storage: Option<&Storage>) -> Result<usize, JsValue> {
        let bag = Self::try_get_bag(storage)?;

        let count = bag.iter().fold(0, |c, (_, count)| c + count);

        let storage = storage.ok_or_else(|| js_exception("Invalid Storage object"))?;

        storage.set_item("bag_count", &count.to_string())?;

        try_dispatch_storage_event()?;

        Ok(count)
    }

    /// Totals the bag as cents
    /// Returns an error on failure
    pub fn try_total_bag(storage: Option<&Storage>) -> Result<i64, JsValue> {
        let bag = Self::try_get_bag(storage)?;

        Ok(bag.iter().fold(0, |c, (product, count)| {
            c + (product.get_price() * (*count as i64))
        }))
    }

    /// Attempts to increment `bag_count` by `value`
    /// Returns an error on failure
    pub fn try_incr_bag_count(storage: Option<&Storage>, value: usize) -> Result<(), JsValue> {
        let existing_count = Self::try_get_bag_count(storage).unwrap_or(0);

        let storage = storage.ok_or_else(|| js_exception("Invalid Storage object"))?;

        storage.set_item("bag_count", &(existing_count + value).to_string())?;

        try_dispatch_storage_event()?;

        Ok(())
    }

    /// Get the value of `bag` from `storage`
    /// Returns an error if `bag` doesn't exist
    pub fn try_get_bag(storage: Option<&Storage>) -> Result<HashMap<Product, usize>, JsValue> {
        let storage = storage.ok_or_else(|| js_exception("Invalid Storage object"))?;

        storage
            .get_item("bag")?
            .as_deref()
            .map(serde_json::from_str)
            .ok_or_else(|| js_exception("Failed to find bag"))?
            .map_err(|_| js_exception("Failed to parse bag"))
    }

    /// Get the value of `bag` from `storage`
    /// Returns an error if `bag` doesn't exist, if it failed to parse
    /// or if an `storage` event failed to send
    pub fn try_add_to_bag(storage: Option<&Storage>, product: Product) -> Result<(), JsValue> {
        let storage = storage.ok_or_else(|| js_exception("Invalid Storage object"))?;

        let mut bag = storage
            .get_item("bag")?
            .as_deref()
            .map_or_else(
                || Ok(HashMap::new()),
                serde_json::from_str::<HashMap<Product, usize>>,
            )
            .map_err(|_| js_exception("Failed to parse bag"))?;

        *bag.entry(product).or_insert(0) += 1;

        let bag = serde_json::to_string(&bag)
            .map_err(|_| js_exception("Failed to convert bag to JSON string"))?;

        storage.set_item("bag", &bag)?;

        Self::try_incr_bag_count(Some(storage), 1)?;

        Ok(())
    }
}
