use bevy::prelude::*;

use super::components::*;
use super::resources::*;

pub fn spawn_flock(
    boid_setting: Res<BoidSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let mesh = meshes.add(Tetrahedron::new(
        Vec3::new(0.25, 0.25, 0.5),
        Vec3::new(-0.25, 0.25, 0.5),
        Vec3::new(0.0, -0.25, 0.5),
        Vec3::new(0.0, 0.0, -0.5),
    ));

    let material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        ..default()
    });

    for _ in 0..boid_setting.count {
        let position = Vec3::random_position();

        commands.spawn((
            BoidAgent::default(),
            StoredVelocity::default(),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
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
        // let neighboring_agents = boid_grid.get_neighbor_agents(cell);
        boid_grid.get_neighbor_boid(cell,&mut boid_agent.neighbors);
        boid_agent.neighbors.retain(|&neighbor| neighbor != entity);
    }
}

pub fn apply_boid_velocity(
    mut query: Query<(&mut Transform, &BoidAgent, &mut StoredVelocity,)>,
    boid_setting: Res<BoidSettings>,
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

            let t = boid_setting.rotation_sharpness * time.delta_secs();

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
    boid_setting: Res<BoidSettings>,
    time: Res<Time>,
){
    for (transform, mut boid_agent, stored_velocity) in &mut query{

        let mut steering = 
            boid_agent.separation  * boid_setting.separation_weight +
            boid_agent.alignment * boid_setting.alignment_weight +
            boid_agent.cohesion * boid_setting.cohesion_weight +
            boid_agent.bound * boid_setting.bound_weight;

        if steering.length_squared() < 0.0001 {
            steering = *transform.forward()
        }
        
        let acceleration = steering.normalize() * boid_setting.acceleration_force;
        let mut next_velocity =
            stored_velocity.0 + acceleration * time.delta_secs();

        next_velocity = next_velocity.clamp_length_max(boid_setting.max_speed);

        boid_agent.velocity = next_velocity;
    }
}

pub fn compute_cohesion_and_separation_and_alignment(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<(&Transform, &StoredVelocity), With<BoidAgent>>,   
    boid_settings: Res<BoidSettings>   
){

    for (transform, mut boid_agent) in &mut query{
        let mut center = Vec3::ZERO;
        let mut force = Vec3::ZERO;
        let mut average_velocity = Vec3::ZERO;
        let mut center_count = 0;
        let mut force_count = 0;
        let mut velocity_count = 0;

        let position = transform.translation;

        for &neighbor in &boid_agent.neighbors{

            if let Ok((neighbor_transform, neighbor_velocity)) = transform_query.get(neighbor){

                let neighbor_position = neighbor_transform.translation;
                let vector = position - neighbor_position;
                let distance_sqr = vector.length_squared();

                if distance_sqr < boid_settings.cohesion_length_sqr {
                    center += neighbor_position;
                    center_count += 1;
                }
                if distance_sqr < boid_settings.separation_length_sqr && distance_sqr > 0.0001 {
                    force += vector / distance_sqr;
                    force_count += 1;
                }

                if distance_sqr < boid_settings.alignment_length_sqr {
                    average_velocity += neighbor_velocity.0;
                    velocity_count += 1;
                }           
            }
        } 

        if force_count > 0{
            force /= force_count as f32;
        }
        boid_agent.separation = force;
        
        if center_count > 0{
            center = (center/center_count as f32 - position).normalize_or_zero();
        }
        boid_agent.cohesion = center;

        if velocity_count > 0{
            average_velocity = (average_velocity / velocity_count as f32).normalize_or_zero();
        } else {
            average_velocity = *transform.forward();
        }
        boid_agent.alignment = average_velocity;

    }
}


pub fn compute_alignment(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<(&Transform, &StoredVelocity), With<BoidAgent>>,
    boid_settings: Res<BoidSettings> 
){
    for (transform, mut boid_agent) in &mut query{
        let mut average_velocity = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok((neighbor_transform, neighbor_velocity)) = transform_query.get(neighbor){
                let offset = neighbor_transform.translation - transform.translation;
                let distance = offset.length_squared();

                if distance > boid_settings.alignment_length_sqr {
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

pub fn compute_cohesion_and_separation(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<&Transform, With<BoidAgent>>,   
    boid_settings: Res<BoidSettings>   
){

    for (transform, mut boid_agent) in &mut query{
        let mut center = Vec3::ZERO;
        let mut force = Vec3::ZERO;
        let mut center_count = 0;
        let mut force_count = 0;
        let position = transform.translation;

        for &neighbor in &boid_agent.neighbors{

            if let Ok(neighbor_transform) = transform_query.get(neighbor){

                let neighbor_position = neighbor_transform.translation;
                let vector = position - neighbor_position;
                let distance_sqr = vector.length_squared();

                if distance_sqr < boid_settings.cohesion_length_sqr {
                    center += neighbor_position;
                    center_count += 1;
                }
                if distance_sqr < boid_settings.separation_length_sqr && distance_sqr > 0.0001 {
                    force += vector / distance_sqr;
                    force_count += 1;
                }
            }
        } 

        if force_count > 0{
            force /= force_count as f32;
        }
        boid_agent.separation = force;
        
        if center_count > 0{
            center = (center/center_count as f32 - position).normalize_or_zero();
        }
        boid_agent.cohesion = center;
    }
}

// pub fn compute_cohesion(
//     mut query: Query<(&Transform, &mut BoidAgent)>, 
//     transform_query: Query<&Transform, With<BoidAgent>>,   
//     boid_settings: Res<BoidSettings>   
// ){
//     for (transform, mut boid_agent) in &mut query{
//         let mut center = Vec3::ZERO;
//         let mut count = 0;
//         let position = transform.translation;

//         for &neighbor in &boid_agent.neighbors{
//             if let Ok(neighbor_transform) = transform_query.get(neighbor){
//                 let neighbor_position = neighbor_transform.translation;
//                 let vector = neighbor_position - position;
//                 let distance = vector.length_squared();

//                 if distance > boid_settings.cohesion_length_sqr {
//                     continue;
//                 }
//                 center += neighbor_position;
//                 count += 1;
//             }
//         } 
        
//         if count > 0{
//             center = (center/count as f32 - position).normalize_or_zero();
//         }
//         boid_agent.cohesion = center;
//     }
// }

// pub fn compute_separation(
//     mut query: Query<(&Transform, &mut BoidAgent)>, 
//     transform_query: Query<&Transform, With<BoidAgent>>,   
//     boid_settings: Res<BoidSettings>   
// ){
//     for (transform, mut boid_agent) in &mut query{
//         let mut force = Vec3::ZERO;
//         let mut count = 0;

//         for &neighbor in &boid_agent.neighbors{
//             if let Ok(neighbor_transform) = transform_query.get(neighbor){
//                 let vector = transform.translation - neighbor_transform.translation;
//                 let distance = vector.length_squared();

//                 if distance > boid_settings.separation_length_sqr || distance < 0.0001 {
//                     continue;
//                 }
//                 force += vector / distance;
//                 count += 1;
//             }
//         } 

//         if count > 0{
//             force /= count as f32;
//         }
//         boid_agent.separation = force;
//     }   
// }

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