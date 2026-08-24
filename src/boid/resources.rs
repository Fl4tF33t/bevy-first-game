use std::collections::HashMap;

use bevy::{math::USizeVec3, prelude::*};

pub const SEPARATION_DISTANCE: f32 = 1.5;
pub const ALIGNMENT_DISTANCE: f32 = 3.5;
pub const COHESION_DISTANCE: f32 = 4.5;
pub const SEPARATION_DISTANCE_SQR: f32 = squared(SEPARATION_DISTANCE);
pub const ALIGNMENT_DISTANCE_SQR: f32 = squared(ALIGNMENT_DISTANCE);
pub const COHESION_DISTANCE_SQR: f32 = squared(COHESION_DISTANCE);

const fn squared(value: f32) -> f32 {
    value * value
}
// const COUNT: usize = 20;
// const SPAWN_AREA: Vec3 = Vec3::new(14.0, 6.0, 14.0);
// const SCALE: f32 = 0.5;
// const BASE_SPEED: f32 = 7.0;
// const SPEED_VARIANCE: f32 = 1.5;
// const FLOCK_BOUNDS_RADIUS: f32 = 30.0;
// const CONSTRAIN_Y: bool = true;
// const CONSTRAINED_Y: f32 = 4.0;

#[derive(Resource)]
pub struct BoidSettings {
    //Spawn
    pub count: usize,
    pub spawn_area: Vec3,
    pub scale: f32,
    
    //Movement
    // pub base_speed: f32,
    // pub speed_variance: f32,
    // pub flock_bounds_radius: f32,

    // //Vertical Constraints
    // pub constrain_y: bool,
    // pub constrained_y: f32,
}


impl BoidSettings {
    pub fn new(count: usize, spawn_area: Vec3, scale: f32) -> Self {
        BoidSettings { 
            count,
            spawn_area,
            scale,
            // count: COUNT,
            // spawn_area: SPAWN_AREA,
            // scale: SCALE,
            // base_speed: BASE_SPEED,
            // speed_variance: SPEED_VARIANCE,
            // flock_bounds_radius: FLOCK_BOUNDS_RADIUS,
            // constrain_y: CONSTRAIN_Y,
            // constrained_y: CONSTRAINED_Y, 
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
    pub fn new(cell_size: f32, dimensions: USizeVec3) -> Self {
        let grid_dimensions= Vec3::new(
            dimensions.x as f32,
            dimensions.y as f32,
            dimensions.z as f32,
        ) * cell_size;
        let origin = -grid_dimensions / 2.0;
        Self {
            cell_size,
            dimensions,
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
pub struct SpatialGrid{
    pub cells: HashMap<IVec3, Vec<Entity>>,
}

impl SpatialGrid{
    pub fn insert(&mut self, key: IVec3, value: Entity){
        self.cells
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

                    if let Some(boid_agents) = self.cells.get(&neighbor) {
                        results.extend(boid_agents);
                    }
                }
            }
        }
        results
    }
}