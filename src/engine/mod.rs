use bevy::prelude::*;
use std::cell::RefCell;
use std::cell::RefMut;
use bevy::window::WindowPlugin;
use console_error_panic_hook;

use crate::commands::AddSquareEvent;
use crate::systems;
use crate::inputs;

thread_local! {
    static APP: RefCell<Option<App>> = RefCell::new(None);
}

pub fn start() {
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#lienzo_engine".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        })
    )
    .add_event::<AddSquareEvent>()
    .add_systems(Startup, systems::setup_camera)
    .add_systems(Startup, systems::check_pipeline_cache_system)
    .add_systems(Update, inputs::handle_input)
    .add_systems(Update, systems::process_add_square_events);
}


pub fn update() {
    APP.with(|app_cell| {
        if let Some(app) = app_cell.borrow_mut().as_mut() {
            app.update();
        } else {
            eprintln!("La aplicación no ha sido inicializada.");
        }
    });
}

pub fn get_app_mut() -> Option<RefMut<'static, Option<App>>> {
    APP.with(|app_cell| {
        Some(app_cell.borrow_mut())
    })
}