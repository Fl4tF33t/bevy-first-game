use bevy::prelude::*;
use avian3d::prelude::*;
use rand::*;

use super::components::BoidAgent;
use super::resources::*;

pub fn update_boid_grid(
    mut boid_grid: ResMut<SpatialGrid>,
    grid_settings: Res<Grid3d>,
    query: Query<(Entity, &Transform), With<BoidAgent>>,
){
    boid_grid.cells.clear();
    for (entity, transform) in &query {
        let key = grid_settings.world_to_cell(transform.translation);
        boid_grid.insert(key, entity);
    }
}

pub fn spawn_flock(
    boid_setting: Res<BoidSettings>,
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let radius = boid_setting.scale;

    for _ in 0..boid_setting.count{
        command.spawn((
            BoidAgent::default(),
            RigidBody::Kinematic,
            Collider::sphere(radius),
            LinearVelocity{
                0: random_vec3_in_limit(boid_setting.spawn_area).normalize()
            },
            Transform{
                translation: random_vec3_in_limit(boid_setting.spawn_area),
                ..default()
            },
            Mesh3d(meshes.add(Sphere{ radius})),
            MeshMaterial3d(materials.add(
                StandardMaterial {
                    base_color: Color::BLACK,
                    ..Default::default()
                })),
        ));
    }
}

pub fn clear_flock(
    query: Query<Entity, With<BoidAgent>>,
    mut command: Commands
) {
    for entity in &query{
        command.entity(entity).despawn();
    }
}

pub fn update_neighbors(
    mut query: Query<(Entity, &Transform, &mut BoidAgent)>,
    grid_settings: Res<Grid3d>,
    boid_grid: Res<SpatialGrid>,
){
    for (entity, transform, mut boid_agent) in &mut query{
        let cell = grid_settings.world_to_cell(transform.translation);
        let neighboring_agents = boid_grid.get_neighbor_agents(cell);
        let agents_without_self: Vec<Entity> = neighboring_agents.into_iter().filter(|&x| x != entity).collect();
        boid_agent.neighbors = agents_without_self;
    }
}

// pub fn compute_steering(
// ){
//     compute_separation();
//     // compute_alignment();
//     // compute_cohesion();
//     // compute_bounds_steer();
//     // compute_obstacle_avoidance();
// }

pub fn compute_separation(
    mut query: Query<(&Transform, &BoidAgent, &mut LinearVelocity)>, 
    transform_query: Query<&Transform, With<BoidAgent>>,     
){
    for (transform, boid_agent, mut linear_velocity) in &mut query{
        let mut force = Vec3::ZERO;
        let mut count = 0;
        for &neighbor in &boid_agent.neighbors{
            if let Ok(neighbor_transform) = transform_query.get(neighbor){
                let vector = transform.translation - neighbor_transform.translation;
                let distance = vector.length_squared();
                info!("Distance {:?}", distance);
                if distance > SEPARATION_DISTANCE_SQR || distance < 0.0001 {
                    continue;
                }
                force += vector / distance;
                count += 1;
            }
        } 
        if count > 0{
            force /= count as f32;
        }
        
        linear_velocity.0 = force;
    }
}

fn random_vec3_in_limit(limit: Vec3) -> Vec3 {
    let mut rng = rng();
    let x = rng.random_range(-limit.x..limit.x);
    let y = rng.random_range(-limit.y..limit.y);
    let z = rng.random_range(-limit.z..limit.z);

    Vec3::new(x, y, z)
}