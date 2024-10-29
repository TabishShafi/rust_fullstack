mod grocery_list {

    use wasm_bindgen::prelude::*;
    use web_sys::window;
    use serde_json;
    
    #[wasm_bindgen]
    pub fn add_item(item: &str) {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        let list = match local_storage.get_item("list") {
            Ok(Some(items)) => items,
            Ok(None) => String::from("[]"),
            Err(_) => String::from("[]"), // Fallback if there's an error accessing local storage
        };

        let mut items: Vec<String> = serde_json::from_str(&list).unwrap();
        items.push(String::from(item));

        let new_list = serde_json::to_string(&items).unwrap();
        local_storage.set_item("list", &new_list).unwrap();
    }

    #[wasm_bindgen]
    pub fn get_items() -> Vec<String> {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();

        let list = match local_storage.get_item("list") {
            Ok(Some(items)) => items,
            Ok(None) => String::from("[]"),
            Err(_) => String::from("[]"), // Fallback if there's an error accessing local storage
        };

        serde_json::from_str(&list).unwrap()
    }
}
