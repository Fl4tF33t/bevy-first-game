use bevy::prelude::*;

use super::components::*;
use super::resources::*;

pub fn spawn_flock(
    boid_setting: Res<BoidSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    for _ in 0..boid_setting.count{
        let position = Vec3::random_position();
        commands.spawn((
            BoidAgent::default(),
            Mesh3d(meshes.add(Tetrahedron::new(
                Vec3::new(0.25, 0.25, 0.5),
                Vec3::new(-0.25, 0.25, 0.5),
                Vec3::new(0.0, -0.25, 0.5),
                Vec3::new(0.0, 0.0, -0.5),
            ))),
            MeshMaterial3d(materials.add(
                StandardMaterial {
                    base_color: Color::BLACK,
                    ..default()
                })
            ),
            Transform::from_translation(position),
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

pub fn rotate_flock(
    mut query: Query<&mut Transform, With<BoidAgent>>,
){
    for mut transform in &mut query {
        let direction = transform.translation * 1.1;
        transform.look_at(direction, Vec3::Y);
    }
}

pub fn compute_bounds(
    mut query: Query<(&Transform, &mut BoidAgent)>,
) {
    let limit = bounds_limit();
    let margin = 5.0;

    for (transform, mut boid_agent) in &mut query {
        let position = transform.translation;
        let mut force = Vec3::ZERO;

        for axis in 0..3 {
            let distance = limit - position[axis].abs();

            if distance < margin {
                let strength = if distance <= 0.0 {
                    2.0
                } else {
                    1.0 - distance / margin
                };

                force[axis] = -position[axis].signum() * strength;
            }
        }
        force *= BOUNDS_WEIGHT;
        boid_agent.bound = force;
    }
}

pub fn update_boid_grid(
    mut boid_grid: ResMut<SpatialGrid>,
    grid_settings: Res<Grid3d>,
    query: Query<(Entity, &Transform), With<BoidAgent>>,
){
    boid_grid.0.clear();
    for (entity, transform) in &query {
        let key = grid_settings.world_to_cell(transform.translation);
        boid_grid.insert(key, entity);
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

pub fn rotate_boid(
    mut query: Query<&mut Transform, With<BoidAgent>>,
    time: Res<Time>,
){ 
    for mut transform in &mut query {
        let direction = transform.translation + *transform.forward();

            // let target_rotation = Transform::from_translation(transform.translation)
            //     .looking_at(direction, Vec3::Y)
            //     .rotation;

            // transform.rotation = transform.rotation.slerp(
            //     target_rotation,
            //     5.0 * time.delta_secs(),
            // );
        transform.look_at(direction, Vec3::Y);
    }
}

pub fn compute_steering(
    mut query: Query<(&mut Transform, &BoidAgent)>,
    time: Res<Time>,
){
    for (mut transform, boid_agent,) in &mut query{
        // let mut velocity = if boid_agent.steering.length_squared() < 0.1 {
        //     *transform.forward()
        // } else {
        //     boid_agent.steering
        // };

        let mut desired_velocity =
            *transform.forward()
            + boid_agent.separation
            + boid_agent.cohesion
            + boid_agent.alignment
            + boid_agent.bound;

        desired_velocity = desired_velocity.normalize();


        // transform.translation += forward;
        // desired_velocity = desired_velocity.normalize();

        let current_velocity = *transform.forward();

        let direction = current_velocity.lerp(
            desired_velocity,
            3.0 * time.delta_secs(),
        );

        transform.look_to(direction, Vec3::Y);

        transform.translation +=
            direction * 5.0 * time.delta_secs();
        
    }
}

// pub fn reset_steering(
//     mut query: Query<&mut BoidAgent>,
// ){
//     for mut boid_agent in &mut query{
//         boid_agent.steering = Vec3::ZERO;
//     }
// }

pub fn compute_alignment(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<&Transform, With<BoidAgent>>, 
){
    for (transform, mut boid_agent) in &mut query{
        let mut force = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok(neighbor_transform) = transform_query.get(neighbor){
                let vector = neighbor_transform.translation - transform.translation;
                let distance = vector.length_squared();
                if distance > ALIGNMENT_DISTANCE_SQR || distance < f32::EPSILON {
                    continue;
                }
                force += *neighbor_transform.forward();
                count += 1;
            }
        } 
        if count > 0{
            force /= count as f32;
            info!("Alignment: {}", force);
        }
        force *= ALIGNMENT_WEIGHT;
        boid_agent.alignment += force;
    }
}

pub fn compute_cohesion(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<&Transform, With<BoidAgent>>,     
){
    for (transform, mut boid_agent) in &mut query{
        let mut force = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok(neighbor_transform) = transform_query.get(neighbor){
                let vector = neighbor_transform.translation - transform.translation;
                let distance = vector.length_squared();
                if distance > COHESION_DISTANCE_SQR || distance < f32::EPSILON {
                    continue;
                }
                force += neighbor_transform.translation;
                count += 1;
            }
        } 
        if count > 0{
            force = (force/count as f32 - transform.translation).normalize_or_zero();
            info!("Cohesion: {}", force);
        }
        force *= COHESION_WEIGHT;
        boid_agent.cohesion = force;
    }
}

pub fn compute_separation(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<&Transform, With<BoidAgent>>,     
){
    for (transform, mut boid_agent) in &mut query{

        
        let mut force = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok(neighbor_transform) = transform_query.get(neighbor){
                let vector = transform.translation - neighbor_transform.translation;
                let distance = vector.length_squared();
                if distance > SEPARATION_DISTANCE_SQR || distance < f32::EPSILON {
                    continue;
                }
                force += vector / distance;
                count += 1;
            }
        } 
        if count > 0{
            force /= count as f32;
            info!("Separation: {}", force);
        }
        force *= SEPARATION_WEIGHT;
        boid_agent.separation = force;
    }   
}