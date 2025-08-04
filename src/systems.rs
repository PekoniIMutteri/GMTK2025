mod camera;
mod enemy;
mod player;

use bevy::prelude::*;

pub struct MySystems;

impl Plugin for MySystems {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
