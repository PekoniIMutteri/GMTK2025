use crate::components::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        markers::Player,
        Collider::circle(10.0),
        Sprite {
            custom_size: Some(Vec2::splat(20.0)),
            ..Default::default()
        },
    ));
}
