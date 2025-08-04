use bevy::prelude::*;

fn main() {
    App::new().add_systems(Startup, hello).run();
}

fn hello() {
    println!("hello");
}
