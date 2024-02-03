use wasm_bindgen::prelude::*;
use web_sys::*;
use web_sys::js_sys::*;

fn to_array(data: String) -> Array {
    let array = Array::new();

    for i in data.chars() {
        array.push(&JsValue::from_str(String::from(i).as_ref()));
    } 

    array
}

#[wasm_bindgen]
pub fn greet(name: &str, num: i32) {
    let mut count = 0;

    loop {
        console::log_1(&JsValue::from_str(format!("{}: {}", name, count).as_ref()));

        if count > num {
            break;
        }

        count += 1;
    }
}

