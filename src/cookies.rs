use js_sys::Object;
use wasm_bindgen::prelude::*;

// TODO other methods
#[wasm_bindgen]
extern "C" {
    pub type Cookies;

    #[wasm_bindgen(method)]
    pub async fn get(this: &Cookies, details: &Object) -> JsValue;

    #[wasm_bindgen(method)]
    pub async fn set(this: &Cookies, details: &Object) -> JsValue;
}