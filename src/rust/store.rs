use js_sys::JSON;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use serde::{Serialize, Deserialize};

fn local_storage()-> web_sys::Storage {
    let window = web_sys::window().unwrap();
    window.local_storage().unwrap().unwrap()
}

#[wasm_bindgen]
extern "C" {
    pub fn model_updated(arg: JsValue);
    pub fn log(s: &str);
}

pub struct Store {
    // local_storage: web_sys::Storage,
    pub items: Vec<Item>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub _type: String,
    pub detail: HashMap<String, Item>,
}

impl Event {
    pub fn new() -> Event {
        Event {
            _type: String::from(""),
            detail: HashMap::new(),
        }
    }

    pub fn trigger(&mut self,_type: &str) {
        self._type = String::from(_type);
        model_updated(JsValue::from_serde(&self).unwrap());
    }

    fn trigger_event(_type: &str, item: Item){
        let mut arg = Event::new();
        arg.detail.insert(String::from("item"),item.clone());
        arg.trigger(_type);
    }
}

impl Store  {
    pub fn init(&mut self, name: &str) {
        self.name = Some(String::from(name));
        self.fetch_from_local_storage();
        Event::new().trigger("fill");
    }

    
    pub fn insert(&mut self, item: &Item) {
        self.items.push(item.clone());
        self.sync_local_storage();
        Event::trigger_event("add", item.clone());
    } 

    pub fn update(&mut self, item: &Item) {
        match self.find(item.clone().id) {
            Some(todo_item) => { 
                todo_item.completed = item.completed;
                todo_item.title = item.title.to_string();
                Event::trigger_event("update", todo_item.clone());
                self.sync_local_storage();
            }
            None => ()
        }
    }

    pub fn batch_update(&mut self, todo_items: Vec<Item>){
        let mut hash_map: HashMap<String, &Item> = HashMap::new();
        for item in todo_items.iter(){
            hash_map.insert(item.id.to_string(), &item);
        }
        for item in self.items.iter_mut(){
            if hash_map.contains_key(&item.id) {
                let new_item = hash_map.get(&item.id);
                item.title = new_item.unwrap().title.to_string();
                item.completed = new_item.unwrap().completed;
            }
        }
        self.sync_local_storage();
        Event::new().trigger("batch_update");
    }

    pub fn delete(&mut self, id: String) {
        Event::trigger_event("delete", self.find(id.clone()).unwrap().clone());
        self.items.retain(|todo_item| todo_item.id != id);
        self.sync_local_storage();
    }

    pub fn batch_delete(&mut self, ids: Vec<String>){
        let hash_set: HashSet<String> = HashSet::from_iter(ids);
        self.items.retain(|todo_item| !hash_set.contains(&todo_item.id));
        self.sync_local_storage();
        Event::new().trigger("batch_delete");
    }

    pub fn find(&mut self, id: String) -> Option<&mut Item>{
        self.items.iter_mut().find(|todo| todo.id == id)
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
                    title ,
                    completed,
                    id,
                };
                self.items.push(temp_item)
            }
        }
        Some(())
    }

}
