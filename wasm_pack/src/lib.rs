use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);
}

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str, num: i32) {
    let mut count = 0;

    loop {
        log!("{}: {}", name, count);
        
        if count > num {
            break;
        }

        count += 1;
    }
}
