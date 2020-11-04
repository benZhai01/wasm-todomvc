use wasm_bindgen::prelude::*;

pub mod store;
use crate::store::{Store };

#[wasm_bindgen]
extern "C" {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(name: & str) {
    get_store().init(name);
    log(&*(get_store().items.len().to_string()));
    
}

static mut STORE: Store = Store {
    items: Vec::new(),
    name: None
};

fn get_store() -> &'static mut Store {
    unsafe {
        return &mut STORE;
    }
}
