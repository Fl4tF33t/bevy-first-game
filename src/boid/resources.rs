use std::collections::HashMap;

use bevy::{math::USizeVec3, prelude::*};

// BOID SETTINGS //
const BOID_COUNT: usize = 1500;
//~Distances
const SEPARATION_DISTANCE: f32 = 1.5;
const ALIGNMENT_DISTANCE: f32 = 3.5;
const COHESION_DISTANCE: f32 = 4.5;
//~Weights
const SEPARATION_WEIGHT: f32 = 2.6;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.2;
const BOUND_WEIGHT: f32 = 5.0;
//~Speeds
const ACCELERATION_FORCE: f32 = 18.0;
const MAX_SPEED: f32 = 7.0;
const ROTATION_SHARPNESS: f32 = 10.0;

// GRID3D
const CELL_SIZE: f32 = 3.0;
const GRID_SIZE: USizeVec3 = USizeVec3::splat(20);

const fn squared(value: f32) -> f32 {
    value * value
}

pub const fn bounds_limit() -> f32 {
    GRID_SIZE.x as f32 * CELL_SIZE / 2.0
}

#[derive(Resource)]
pub struct BoidSettings {
    //Spawn
    pub count: usize,

    // Distances
    pub separation_length_sqr: f32,
    pub alignment_length_sqr: f32,
    pub cohesion_length_sqr: f32,

    // Weights
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub bound_weight: f32,

    // Speeds
    pub acceleration_force: f32,
    pub max_speed: f32,
    pub rotation_sharpness: f32,
}

impl BoidSettings {
    pub fn new() -> Self {
        BoidSettings {
            count: BOID_COUNT,

            separation_length_sqr: squared(SEPARATION_DISTANCE),
            alignment_length_sqr: squared(ALIGNMENT_DISTANCE),
            cohesion_length_sqr: squared(COHESION_DISTANCE),

            separation_weight: SEPARATION_WEIGHT,
            alignment_weight: ALIGNMENT_WEIGHT,
            cohesion_weight: COHESION_WEIGHT,
            bound_weight: BOUND_WEIGHT,

            acceleration_force: ACCELERATION_FORCE,
            max_speed: MAX_SPEED,
            rotation_sharpness: ROTATION_SHARPNESS,
        }
    }
}

#[derive(Resource)]
pub struct Grid3d{
    pub cell_size: f32,
    pub dimensions: USizeVec3,
    origin: Vec3,
}

impl Grid3d {
    pub fn new() -> Self {
        let grid_dimensions = GRID_SIZE.as_vec3() * CELL_SIZE;
        let origin = -grid_dimensions / 2.0;
        Self {
            cell_size: CELL_SIZE,
            dimensions: GRID_SIZE,
            origin,
        }
    }

    pub fn world_to_cell(&self, position: Vec3) -> IVec3 {
        ((position - self.origin) / self.cell_size)
            .floor()
            .as_ivec3()
    }

    pub fn cell_to_world(&self, cell: IVec3) -> Vec3 {
        self.origin + (cell.as_vec3() + Vec3::splat(0.5)) * self.cell_size
    }
}


#[derive(Resource, Default)]
pub struct SpatialGrid (pub HashMap<IVec3, Vec<Entity>>);

impl SpatialGrid{
    pub fn insert(&mut self, key: IVec3, value: Entity){
        self.0
            .entry(key)
            .or_default()
            .push(value);
    }

    pub fn get_neighbor_boid(&self, cell: IVec3, results: &mut Vec<Entity>) {
        results.clear();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let neighbor_cell = cell + IVec3::new(x, y, z);

                    if let Some(boid_agents) = self.0.get(&neighbor_cell) {
                        results.extend(boid_agents);
                    }
                }
            }
        }
    }
}