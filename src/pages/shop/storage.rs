use js_sys::Error;
use leptos::{leptos_dom::helpers, tachys::html::event::Event};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Storage;

use super::item::Product;

pub fn get_storage() -> Option<Storage> {
    helpers::window().local_storage().ok()?
}

pub fn try_dispatch_storage_event() -> Result<(), JsValue> {
    let window = helpers::window();
    let event = Event::new("storage")?;

    let _ = window.dispatch_event(&event)?;

    Ok(())
}

pub struct Bag;

impl Bag {
    pub fn try_get_item_count_from_storage(storage: Option<Storage>) -> Result<usize, JsValue> {
        let storage =
            storage.ok_or_else(|| Into::<JsValue>::into(Error::new("Invalid storage object")))?;
        storage.get_item("bag_count")?.map_or(Ok(0), |v| {
            v.parse::<usize>().map_err(|_| {
                let _ = storage.delete("bag_count");
                JsValue::null()
            })
        })
    }

    pub fn get_item_count_from_storage_or_default(storage: Option<Storage>) -> usize {
        Self::try_get_item_count_from_storage(storage).unwrap_or(0)
    }

    /*
    pub fn try_get_bag_items(storage: Option<Storage>) -> Result<HashMap<Product, usize>, JsValue> {
        let storage = storage.ok_or_else(|| JsValue::from("Invalid Storage object"))?;

        let items: HashMap<Product, usize> = storage
            .get_item("bag")?
            .as_deref()
            .map(serde_json::from_str)
            .ok_or_else(|| JsValue::from("`items` doesn't exist in storage"))?
            .map_err(|_| JsValue::from("Failed to parse items"))?;

        Ok(items)
    }*/

    pub fn try_add_bag_item(storage: Option<Storage>, product: Product) -> Result<(), JsValue> {
        let storage =
            storage.ok_or_else(|| Into::<JsValue>::into(Error::new("Invalid Storage object")))?;

        if let Ok(mut items) = storage.get_item("bag")?.as_deref().map_or_else(
            || Ok(HashMap::new()),
            serde_json::from_str::<HashMap<Product, usize>>,
        ) {
            *items.entry(product).or_insert(0) += 0;

            let items = serde_json::to_string(&items)
                .map_err(|_| JsValue::from("Failed to convert `items` to JSON string"))?;

            storage.set_item("bag", &items)?;

            try_dispatch_storage_event()?;

            Ok(())
        } else {
            Err(JsValue::from("Failed to parse items in Bag"))
        }
    }
}
