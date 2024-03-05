use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(a: &str) -> String {
  format!("Hello, {}!", a)
}