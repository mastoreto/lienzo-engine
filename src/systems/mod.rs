use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::render_resource::PipelineCache;
use crate::{commands::AddSquareEvent};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn process_add_square_events(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut event_reader: EventReader<AddSquareEvent>,
) {
    for event in event_reader.iter() {
        let options = &event.0;

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
}

pub fn check_pipeline_cache_system(
    pipeline_cache: Option<Res<PipelineCache>>,
) {
    if pipeline_cache.is_some() {
        println!("PipelineCache está inicializado.");
    } else {
        println!("PipelineCache no está inicializado.");
    }
}