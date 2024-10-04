use wasm_bindgen::JsValue;
use serde::Deserialize;
use serde_wasm_bindgen;

use bevy::prelude::*;

use crate::entities::SquareOptions;
use crate::engine;

#[derive(Clone)]
pub struct AddSquareEvent(pub SquareOptions);

impl bevy::prelude::Event for AddSquareEvent {}

pub fn add_square(options: &JsValue) {
    let options: SquareOptions = serde_wasm_bindgen::from_value(options.clone()).unwrap();

    if let Some(app) = engine::get_app_mut() {
        let mut event_writer = app.world.get_resource_mut::<Events<AddSquareEvent>>().unwrap();
        event_writer.send(AddSquareEvent(options));
    }
}