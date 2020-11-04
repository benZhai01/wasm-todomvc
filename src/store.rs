use js_sys::JSON;
use wasm_bindgen::prelude::*;

fn local_storage()-> web_sys::Storage {
    let window = web_sys::window().unwrap();
    window.local_storage().unwrap().unwrap()
}


pub struct Store {
    // local_storage: web_sys::Storage,
    pub items: Vec<Item>,
    pub name: Option<String>,
}

pub struct Item {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

impl Store  {
    pub fn init(&mut self, name: &str) {
        self.name = Some(String::from(name));
        self.fetch_from_local_storage();
    }

    fn fetch_from_local_storage(&mut self) -> Option<()>{
        if let Ok(Some(value)) = local_storage().get_item(self.name.as_ref().unwrap()) {
            let data = JSON::parse(&value).ok()?;
            let iter = js_sys::try_iter(&data).ok()??;
            for item in iter {
                let item = item.ok()?;
                let item_arr: &js_sys::Array = wasm_bindgen::JsCast::dyn_ref(&item)?;
                let title = item_arr.shift().as_string()?;
                let completed = item_arr.shift().as_bool()?;
                let id = item_arr.shift().as_string()?;
                let temp_item = Item {
                    title,
                    completed,
                    id,
                };
                self.items.push(temp_item)
            }
        }
        Some(())
    }

    pub fn insert(&mut self, item: Item) {
        self.items.push(item);
        self.sync_local_storage();
    } 

    fn sync_local_storage(&mut self) {
        let array = js_sys::Array::new();
        for item in &self.items {
            let child = js_sys::Array::new();
            child.push(&JsValue::from(&item.title));
            child.push(&JsValue::from(item.completed));
            child.push(&JsValue::from(&item.id));

            array.push(&JsValue::from(child));
        }
        if let Ok(storage_string) = JSON::stringify(&JsValue::from(array)) {
            let storage_string: String = storage_string.into();
            local_storage()
                .set_item(self.name.as_ref().unwrap(), &storage_string)
                .unwrap();
        }
    }
}