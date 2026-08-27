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
            StoredVelocity::default(),
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

pub fn apply_boid_velocity(
    mut query: Query<(&mut Transform, &BoidAgent, &mut StoredVelocity,)>,
    time: Res<Time>,
) {
    for (mut transform, boid_agent, mut stored_velocity) in &mut query {
        let velocity = boid_agent.velocity;

        stored_velocity.0 = velocity;
        if velocity.length_squared() > 0.01 {
            transform.translation += velocity * time.delta_secs();


            let target_rotation =
                Quat::from_rotation_arc(
                    Vec3::NEG_Z,
                    velocity.normalize(),
                );

            let t =
                ROTATION_SHARPNESS * time.delta_secs();

            transform.rotation =
                transform.rotation.slerp(
                    target_rotation,
                    t,
                );
        }
    }
}

pub fn compute_steering(
    mut query: Query<(&Transform, &mut BoidAgent, &StoredVelocity)>,
    time: Res<Time>,
){
    for (transform, mut boid_agent, stored_velocity) in &mut query{

        let mut steering = 
            boid_agent.separation  * SEPARATION_WEIGHT +
            boid_agent.alignment * ALIGNMENT_WEIGHT +
            boid_agent.cohesion *COHESION_WEIGHT +
            boid_agent.bound * BOUNDS_WEIGHT;

        if steering.length_squared() < 0.0001 {
            steering = *transform.forward()
        }
        
        let acceleration = steering.normalize() * ACCELERATION_FORCE;
        let mut next_velocity =
            stored_velocity.0 + acceleration * time.delta_secs();

        next_velocity = next_velocity.clamp_length_max(MAX_SPEED);

        boid_agent.velocity = next_velocity;

        info!("Next Velocity: {}", next_velocity);
        // let mut desired_velocity =
        //     *transform.forward()
        //     + boid_agent.separation
        //     + boid_agent.cohesion
        //     + boid_agent.alignment
        //     + boid_agent.bound;

        // desired_velocity = desired_velocity.normalize();


        // // transform.translation += forward;
        // // desired_velocity = desired_velocity.normalize();

        // let current_velocity = *transform.forward();

        // let direction = current_velocity.lerp(
        //     desired_velocity,
        //     3.0 * time.delta_secs(),
        // );

        // transform.look_to(direction, Vec3::Y);

        // transform.translation +=
        //     direction * 5.0 * time.delta_secs();
        
    }
}


pub fn compute_alignment(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<(&Transform, &StoredVelocity)>, 
){
    for (transform, mut boid_agent) in &mut query{
        let mut average_velocity = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok((neighbor_transform, neighbor_velocity)) = transform_query.get(neighbor){
                let offset = neighbor_transform.translation - transform.translation;
                let distance = offset.length_squared();

                if distance > ALIGNMENT_DISTANCE_SQR {
                    continue;
                }
                average_velocity += neighbor_velocity.0;
                count += 1;
            }
        } 


        if count > 0{
            average_velocity = (average_velocity / count as f32).normalize_or_zero();
        } else {
            average_velocity = *transform.forward();
        }
        boid_agent.alignment = average_velocity;
    }
}

pub fn compute_cohesion(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<&Transform, With<BoidAgent>>,     
){
    for (transform, mut boid_agent) in &mut query{
        let mut center = Vec3::ZERO;
        let mut count = 0;
        let position = transform.translation;

        for &neighbor in &boid_agent.neighbors{
            if let Ok(neighbor_transform) = transform_query.get(neighbor){
                let neighbor_position = neighbor_transform.translation;
                let vector = neighbor_position - position;
                let distance = vector.length_squared();

                if distance > COHESION_DISTANCE_SQR {
                    continue;
                }
                center += neighbor_position;
                count += 1;
            }
        } 
        
        if count > 0{
            center = (center/count as f32 - position).normalize_or_zero();
        }
        boid_agent.cohesion = center;
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
        boid_agent.separation = force;
    }   
}

pub fn compute_bounds(
    mut query: Query<(&Transform, &mut BoidAgent)>,
) {
    let limit = bounds_limit();
    let inner_radius = limit * 0.85;

    for (transform, mut boid_agent) in &mut query {
        let offset = transform.translation;
        let distance = offset.length();

        if distance <= inner_radius {
            boid_agent.bound = Vec3::ZERO;
            continue;
        }

        let strength = (
            (distance - inner_radius) /
            (limit - inner_radius)
        ).clamp(0.0, 1.0);

        boid_agent.bound =
            (-transform.translation).normalize_or_zero()
            * strength;
    }
}