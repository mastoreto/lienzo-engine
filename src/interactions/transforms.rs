
use bevy::prelude::*;
use crate::entities::components::{Movable, Resizable};

pub fn rotate_system(mut query: Query<&mut Transform, With<Rotatable>>) {
    // Lógica para rotar entidades
}

pub fn resize_system(mut query: Query<&mut Transform, With<Resizable>>) {
    // Lógica para redimensionar entidades
}

pub fn drag_system(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    // Otros recursos necesarios
) {
    // Lógica para arrastrar entidades
}
