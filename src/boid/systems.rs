use bevy::prelude::*;
use avian3d::prelude::*;

use super::components::*;
use super::resources::*;

pub fn spawn_flock(
    boid_setting: Res<BoidSettings>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    let radius = boid_setting.scale;
    
    for _ in 0..boid_setting.count{
        let velocity = Vec3::random() * 10.0;
        let position = Vec3::random() * 20.0;
        
        commands.spawn((
            BoidAgent::default(),
            RigidBody::Kinematic,
            Collider::sphere(radius),
            LinearVelocity(velocity),
            Transform::from_translation(position),
        ))
            .with_children(|parent| {
                parent.spawn((
                Mesh3d(meshes.add(Cone::default())),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::BLACK,
                    ..default()
                })),
                Transform::from_rotation(
                    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
                ),
            ));
        });
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
    mut query: Query<(&mut Transform, &LinearVelocity), With<BoidAgent>>,
    time: Res<Time>,
){ 

    for (mut transform, linear_velocity) in &mut query{
        let velocity = linear_velocity.0;
        if velocity.length_squared() < 0.1{
            continue;
        }

        let direction = transform.translation + velocity;
        // transform.look_to(Vec3::ZERO, Vec3::Y);
        transform.look_to(direction, Dir3::Y);
    }
}

pub fn compute_steering(
    mut query: Query<(&Transform, &mut BoidAgent, &mut LinearVelocity)>,
    time: Res<Time>,
){
    for (transform, mut boid_agent, mut linear_velocity) in &mut query{
        let velocity = if boid_agent.steering.length_squared() < 0.1 {
            *transform.forward() * time.delta_secs() * 20.0
        } else {
            boid_agent.steering * time.delta_secs() * 20.0
        };
        
        linear_velocity.0 = velocity;
        boid_agent.steering = Vec3::ZERO;
    }
}

pub fn compute_alignment(
    mut query: Query<(&Transform, &mut BoidAgent)>, 
    transform_query: Query<(&Transform, &LinearVelocity)>, 
){
    for (transform, mut boid_agent) in &mut query{
        let mut force = Vec3::ZERO;
        let mut count = 0;

        for &neighbor in &boid_agent.neighbors{
            if let Ok((neighbor_transform, linear_velocity)) = transform_query.get(neighbor){
                let vector = transform.translation - neighbor_transform.translation;
                let distance = vector.length_squared();
                if distance > ALIGNMENT_DISTANCE_SQR || distance < f32::EPSILON {
                    continue;
                }
                force += linear_velocity.0;
                count += 1;
            }
        } 
        if count > 0{
            force /= count as f32;
        }
        boid_agent.steering += force;
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
                let vector = transform.translation - neighbor_transform.translation;
                let distance = vector.length_squared();
                if distance > COHESION_DISTANCE_SQR || distance < f32::EPSILON {
                    continue;
                }
                force += neighbor_transform.translation;
                count += 1;
            }
        } 
        if count > 0{
            force = (force/count as f32 - transform.translation).normalize();
        }
        boid_agent.steering += force;
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
        }
        boid_agent.steering += force;
    }   
}