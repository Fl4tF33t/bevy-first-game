use std::collections::HashSet;
use std::hash::Hash;
use bevy::prelude::*;
use crate::asset_importer::resources::AssetImporter;
use super::components::*;
use avian3d::prelude::*;
use avian3d::math::*;

const PLAYER_SPEED: f32 = 5.0;

pub fn spawn_player(
    mut commands: Commands,
    imported_assets: Res<AssetImporter>,
) {
    commands.spawn((
        WorldAssetRoot(imported_assets.player_scene.clone()),
        Transform::from_xyz(0.0, 5.0, 0.0),
        Player,
        // CharacterController::default(),
        RigidBody::Kinematic,
        Collider::capsule(1.0, 2.0),
        Motor {
            speed: PLAYER_SPEED,
            ..default()
        },
    ));
}

pub fn read_movement_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&mut Motor, With<Player>>,
) {
    let mut motor = player.into_inner();
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

pub fn apply_kinematic_gravity(
    mut query: Query<&mut Motor>,
    time: Res<Time>,
) {
    for mut motor in query.iter_mut() {
        if motor.grounded {
            motor.vertical_velocity = 0.0;
        } else {
            motor.vertical_velocity -= 9.81 * time.delta_secs();
        }
    }
}

pub fn entity_motor(
    // player: Single<(Entity, &Collider, &mut Motor, &mut Transform), With<Player>>,
    mut query: Query<(Entity, &Collider, &mut Motor, &mut Transform)>,
    move_and_slide: MoveAndSlide,
    time: Res<Time>,
) {
    for (entity, collider, mut motor, mut transform) in query.iter_mut() {
        let mut velocity = motor.direction * motor.speed;
        velocity.y = motor.vertical_velocity;
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);
        let mut collisions = HashSet::new();
        let mut normals = Vec::new();

        let out = move_and_slide.move_and_slide(
            collider,
             transform.translation.adjust_precision(),
             transform.rotation.adjust_precision(),
            velocity,
            time.delta(),
            &MoveAndSlideConfig::default(),
            &filter,
            |hit| {
                {
                    collisions.insert(hit.entity);
                    normals.push(hit.normal.clone());
                };
                MoveAndSlideHitResponse::Accept
            },
        );
        
        transform.translation = out.position.f32();
        motor.direction = out.projected_velocity;
        // info!("Colliding with entities: {:?}", collisions);
        for normal in normals.iter() {
            if normal.y > 0.7 {
                motor.grounded = true;
                motor.vertical_velocity = 0.0;
            }
        }
        info!("Gravity applied: {}", motor.vertical_velocity);
    }
}