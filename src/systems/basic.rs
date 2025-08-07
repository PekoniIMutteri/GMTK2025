use avian2d::prelude::*;
use bevy::prelude::*;

use crate::components::basic::*;

pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement)
            .add_systems(Update, handle_input);
    }
}

fn apply_movement(query: Query<(&mut Movement, &mut LinearVelocity)>) {
    for (mut movement, mut velocity) in query {
        if movement.enabled {
            **velocity = movement.desired_direction * movement.max_speed;
            movement.desired_direction = Vec2::ZERO;
            println!("{}", velocity.length());
        }
    }
}

fn handle_input(query: Query<(&mut Movement, &Input)>, keyboard: Res<ButtonInput<KeyCode>>) {
    for (mut movement, input) in query {
        movement.desired_direction = input.input_vec(&keyboard);
    }
}
