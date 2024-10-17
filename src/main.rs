use bevy::prelude::*;
mod camera;
mod globals;
mod scene;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<states::GameState>()
        .add_systems(Startup, camera::setup)
        .add_systems(Startup, scene::setup_scene)
        .add_systems(Startup, states::startup_system)
        .add_systems(Update, states::player_movement_system)
        .add_systems(Update, camera::camera_follow_system)
        .run();
}
