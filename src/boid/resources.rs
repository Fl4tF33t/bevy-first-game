use std::collections::HashMap;

use bevy::{math::USizeVec3, prelude::*};

// BOID SETTINGS
pub const BOID_COUNT: usize = 13;
pub const DIMENSIONS: USizeVec3 = USizeVec3::splat(10);

// GRID3D
pub const CELL_SIZE: f32 = 5.0;

pub const SEPARATION_DISTANCE: f32 = 1.5;
pub const ALIGNMENT_DISTANCE: f32 = 3.5;
pub const COHESION_DISTANCE: f32 = 4.5;
pub const SEPARATION_DISTANCE_SQR: f32 = squared(SEPARATION_DISTANCE);
pub const ALIGNMENT_DISTANCE_SQR: f32 = squared(ALIGNMENT_DISTANCE);
pub const COHESION_DISTANCE_SQR: f32 = squared(COHESION_DISTANCE);

pub const SEPARATION_WEIGHT: f32 = 1.6;
pub const ALIGNMENT_WEIGHT: f32 = 1.0;
pub const COHESION_WEIGHT: f32 = 1.2;
pub const BOUNDS_WEIGHT: f32 = 5.0;

const fn squared(value: f32) -> f32 {
    value * value
}

pub const fn bounds_limit() -> f32 {
    DIMENSIONS.x as f32 * CELL_SIZE / 2.0
}

#[derive(Resource)]
pub struct BoidSettings {
    //Spawn
    pub count: usize,
    pub scale: f32,
}

impl BoidSettings {
    pub fn new() -> Self {
        BoidSettings { 
            count: BOID_COUNT,
            scale: 0.5,
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
        let grid_dimensions = DIMENSIONS.as_vec3() * CELL_SIZE;
        let origin = -grid_dimensions / 2.0;
        Self {
            cell_size: CELL_SIZE,
            dimensions: DIMENSIONS,
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

    pub fn get_neighbor_agents(&self, cell: IVec3) -> Vec<Entity> {
        let mut results = Vec::new();

        for x in -1..= 1 {
            for y in -1..= 1 {
                for z in -1..= 1 {
                    let neighbor = cell + IVec3::new(x, y, z);

                    if let Some(boid_agents) = self.0.get(&neighbor) {
                        results.extend(boid_agents);
                    }
                }
            }
        }
        results
    }
}