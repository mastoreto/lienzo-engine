use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SquareOptions {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: String,
}

pub fn spawn_square(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>, options: SquareOptions) {
    let color = match options.color.to_lowercase().as_str() {
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        "yellow" => Color::YELLOW,
        "purple" => Color::PURPLE,
        _ => Color::WHITE,
    };

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad::new(Vec2::new(options.size, options.size))))
            .into(),
        material: materials.add(ColorMaterial::from(color)),
        transform: Transform::from_translation(Vec3::new(options.x, options.y, 0.0)),
        ..Default::default()
    });
}
