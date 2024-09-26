use bevy::prelude::*;
use crate::entities::Position;
use crate::entities::Movable;


pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Position, (With<Movable>,)>,
) {
    for mut position in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            position.x -= 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            position.x += 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            position.y += 2.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            position.y -= 2.0;
        }
    }
}
