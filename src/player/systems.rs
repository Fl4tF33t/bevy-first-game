use bevy::prelude::*;
use crate::asset_importer::resources::AssetImporter;
use super::components::*;
use avian3d::prelude::*;

const PLAYER_SPEED: f32 = 5.0;

pub fn spawn_player(
    mut commands: Commands,
    imported_assets: Res<AssetImporter>,
) {
    commands.spawn((
        WorldAssetRoot(imported_assets.player_scene.clone()),
        Transform::from_xyz(0.0, 5.0, 0.0),
        Player,
        RigidBody::Dynamic,
        Collider::capsule(1.0, 2.0),
        Motor {
            speed: PLAYER_SPEED,
            ..default()
        },
    ));
}

pub fn motor_system(
    mut query: Query<(&mut Transform, &Motor)>,
    time: Res<Time>,
) {
    for (mut transform, motor) in query.iter_mut() {
        let movement = motor.direction * motor.speed * time.delta_secs();
        transform.translation += movement;
    }
}

pub fn read_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Motor, With<Player>>,
) {
    let mut motor = query.single_mut().expect("Could not find player");
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    motor.direction = direction.normalize_or_zero();   
}
