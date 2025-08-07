mod basic;
mod camera;
mod enemy;
mod player;
mod setup;

use bevy::prelude::*;

pub struct MySystems;

impl Plugin for MySystems {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin)
            .add_plugins(setup::SetupPlugin)
            .add_plugins(basic::BasicPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
