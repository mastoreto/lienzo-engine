use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use crate::systems;
use crate::inputs;

#[wasm_bindgen]
pub fn start_engine() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, systems::move_entities)
        .add_systems(Update,inputs::handle_input)
        .run();
        
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
