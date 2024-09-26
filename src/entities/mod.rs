use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
pub struct Movable;

#[derive(Bundle)]
pub struct EntityBundle {
    pub position: Position,
    pub movable: Movable,
    pub sprite_bundle: SpriteBundle,
}

pub fn create_entity(commands: &mut Commands, position: Position) {
    commands.spawn(EntityBundle {
        position,
        movable: Movable,
        sprite_bundle: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),  // Usamos `custom_size` en lugar de `size`
                color: Color::srgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            transform: Transform::from_xyz(position.x, position.y, 0.0).clone(),
            ..Default::default()
        },
    });
}
