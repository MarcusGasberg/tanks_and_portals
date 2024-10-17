use bevy::{prelude::*, render::camera::ScalingMode};

use crate::states::Player;

#[derive(Component)]
pub struct MainCamera;

pub fn setup(mut commands: Commands) {
    let camera = Camera3dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(16.0),
            ..default()
        }
        .into(),
        transform: Transform::from_xyz(5.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    commands.spawn(camera).insert(MainCamera);
}

pub fn camera_follow_system(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.z = player_transform.translation.z;
        }
    }
}
