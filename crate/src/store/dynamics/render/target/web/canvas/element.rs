use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct Canvas {
    width: f64,
    height: f64,
}
