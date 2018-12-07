extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate serde_json;

mod utils;
mod transform;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::throw_str;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn transform(doc: JsValue, old_schema: JsValue, new_schema: JsValue) -> JsValue {
    if doc.is_object() && old_schema.is_object() && new_schema.is_object() {
        let doc_conv = doc.into_serde().unwrap();
        let old_conv = old_schema.into_serde().unwrap();
        let new_conv = new_schema.into_serde().unwrap();

        let result = transform::transform(doc_conv, old_conv, new_conv);

        JsValue::from_serde(&result).unwrap()
    } else {
        throw_str("doc, old and new schema need to be objects!");
    }
}
