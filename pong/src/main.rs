use bevy::prelude::*;

fn spawn_ball(mut commands: Commands) {
    println!("Spawning ball ...");
    commands.spawn_empty();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_ball)
        .run();
}