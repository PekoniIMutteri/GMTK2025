use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Movement {
    pub desired_direction: Vec2,
    pub enabled: bool,
    pub max_speed: f32,
}

impl Movement {
    pub fn new(max_speed: f32) -> Self {
        Movement {
            desired_direction: Vec2::ZERO,
            enabled: true,
            max_speed,
        }
    }
}

#[derive(Component)]
pub struct Input {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    //pub action: KeyCode,
}

impl Input {
    pub fn input_vec(&self, keyboard: &Res<ButtonInput<KeyCode>>) -> Vec2 {
        let mut vector = Vec2::ZERO;
        if keyboard.pressed(self.left) {
            vector.x -= 1.0;
        }
        if keyboard.pressed(self.right) {
            vector.x += 1.0;
        }
        if keyboard.pressed(self.up) {
            vector.y += 1.0;
        }
        if keyboard.pressed(self.down) {
            vector.y -= 1.0;
        }
        vector.normalize_or_zero()
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            //left: KeyCode::KeyA,
        }
    }
}
