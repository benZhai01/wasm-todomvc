use wasm_bindgen::prelude::*;

pub mod store;
use crate::store::{Store, Item };

static mut STORE: Store = Store {
    items: Vec::new(),
    name: None
};

fn get_store() -> &'static mut Store {
    unsafe {
        return &mut STORE;
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(name: & str) {
    get_store().init(name);
}

#[wasm_bindgen]
pub fn get_items() -> JsValue{
    JsValue::from_serde(&get_store().items).unwrap()
}

#[wasm_bindgen]
pub fn add_item(val: &JsValue) {
    let todo_item: Item = val.into_serde().unwrap();
    get_store().insert(&todo_item);
}

#[wasm_bindgen]
pub fn update_item(val: &JsValue){
    let todo_item: Item = val.into_serde().unwrap();
    get_store().update(&todo_item);
}

#[wasm_bindgen]
pub fn update_items(val: &JsValue){
    let todo_items: Vec<Item> = val.into_serde().unwrap();
    get_store().batch_update(todo_items);
}

#[wasm_bindgen]
pub fn delete_item(id: String) {
    get_store().delete(id);
}

#[wasm_bindgen]
pub fn delete_items(val: &JsValue) {
    let ids: Vec<String> = val.into_serde().unwrap();
    get_store().batch_delete(ids);
}