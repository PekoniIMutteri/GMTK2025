use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::Fifo,
                resolution: WindowResolution::new(640.0, 360.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins(systems::MySystems)
        .run();
}
