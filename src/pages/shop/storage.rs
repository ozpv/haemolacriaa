use leptos::{leptos_dom::helpers, tachys::html::event::Event};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::Storage;

use super::item::Product;

#[wasm_bindgen]
pub fn get_storage() -> Option<Storage> {
    helpers::window().local_storage().ok()?
}

#[wasm_bindgen]
pub struct Bag;

#[wasm_bindgen]
impl Bag {
    fn try_dispatch_storage_event() -> Result<(), JsValue> {
        let window = helpers::window();
        let event = Event::new("storage")?;

        window.dispatch_event(&event)?;

        Ok(())
    }

    pub fn try_get_item_count_from_storage(storage: Option<Storage>) -> Result<usize, JsValue> {
        let storage = storage.ok_or_else(|| JsValue::from("Invalid storage object"))?;
        storage.get_item("count")?.map_or(Ok(0), |v| {
            v.parse::<usize>().map_err(|_| {
                let _ = storage.delete("count");
                JsValue::null()
            })
        })
    }

    pub fn get_item_count_from_storage_or_default(storage: Option<Storage>) -> usize {
        Self::try_get_item_count_from_storage(storage).unwrap_or(0)
    }

    pub fn try_get_bag_items(storage: Option<Storage>) -> Result<Vec<Product>, JsValue> {
        let storage = storage.ok_or_else(|| JsValue::from("Invalid Storage object"))?;

        let items: Vec<Product> = storage
            .get_item("items")?
            .as_deref()
            .map(serde_json::from_str)
            .ok_or_else(|| JsValue::from("`items` doesn't exist in storage"))?
            .map_err(|_| JsValue::from("Failed to parse items"))?;

        Ok(items)
    }

    pub fn try_add_bag_item(storage: Option<Storage>, product: Product) -> Result<(), JsValue> {
        let storage = storage.ok_or_else(|| JsValue::from("Invalid Storage object"))?;

        if let Ok(mut items) = storage
            .get_item("items")?
            .as_deref()
            .map(serde_json::from_str::<Vec<Product>>)
            .unwrap_or_else(|| Ok(Vec::<Product>::new()))
        {
            items.push(product);

            let items = serde_json::to_string(&items)
                .map_err(|_| JsValue::from("Failed to convert `items` to JSON string"))?;

            storage.set_item("items", &items)?;

            Self::try_dispatch_storage_event()?;

            Ok(())
        } else {
            Err(JsValue::from("Failed to parse items in Bag"))
        }
    }
}
