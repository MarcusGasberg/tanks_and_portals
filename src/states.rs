use bevy::{
    ecs::bundle,
    math::{I64Vec3, VectorSpace},
    prelude::*,
};

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

#[derive(Component)]
pub struct Player {
    name: String,
}

#[derive(Component)]
pub struct Score {
    value: usize,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    health: Health,
    score: Score,
    sprite: PbrBundle,
    velocity: Velocity,
    speed: Speed,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Enemy {
    name: String,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    health: Health,
}

#[derive(Component)]
pub struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
pub struct Speed {
    current: f32,
    max: f32,
}

#[derive(Resource, Default)]
pub struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<Player>,
}

#[derive(Resource)]
pub struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

pub fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    // Create our game rules resource
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    commands.spawn(PlayerBundle {
        marker: Player {
            name: "Alice".to_string(),
        },
        health: Health {
            current: 100,
            max: 100,
        },
        score: Score { value: 0 },
        sprite: PbrBundle {
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(1.5, 0.5, 1.5),
            ..default()
        },
        velocity: Velocity(Vec3::ZERO),
        speed: Speed {
            max: 2.,
            current: 2.,
        },
    });

    commands.spawn_batch(vec![EnemyBundle {
        marker: Enemy {
            name: "Bad guy".to_string(),
        },
        health: Health {
            current: 100,
            max: 100,
        },
    }]);

    game_state.total_players = 1;
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity, &Speed), With<Player>>,
) {
    for (mut transform, mut velocity, speed) in query.iter_mut() {
        // Handle input for basic movement
        velocity.0 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.0 += Vec3::new(-1.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.0 += Vec3::new(1.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.0 += Vec3::new(-1.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.0 += Vec3::new(1.0, 0.0, -1.0);
        }

        // Normalize velocity to prevent faster diagonal movement
        if velocity.0 != Vec3::ZERO {
            velocity.0 = velocity.0.normalize();
        }

        // Update position based on velocity and players speed
        transform.translation += velocity.0 * time.delta_seconds() * speed.current;
    }
}
