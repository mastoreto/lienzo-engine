mod engine;
mod entities;
mod inputs;
mod systems;
mod commands;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start_engine() {
    engine::start();
}

#[wasm_bindgen]
pub fn update() {
    engine::update();
}

#[wasm_bindgen]
pub fn add_square(options: &JsValue) {
    commands::add_square(options);
}
