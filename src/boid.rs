mod components;
mod systems;
mod resources;
mod debug;

use bevy::{math::USizeVec3, prelude::*};
use resources::*;
use systems::*;
use debug::*;

pub struct BoidPlugin;
impl Plugin for BoidPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BoidSettings::new(3, Vec3::new(1.0, 1.0, 1.0), 0.5))
            .insert_resource(Grid3d::new(10.0, USizeVec3 { x: 10, y: 10, z: 10 }))
            .insert_resource(SpatialGrid::default())
            .add_systems(Startup, (
                clear_flock,
                spawn_flock,
            ).chain())
            .add_systems(Update, (
                update_boid_grid,
                update_neighbors,
                compute_separation,
            ))
            .add_systems(Update, (
                draw_grid_cells,
                draw_occupied_cells,
                draw_lines_to_neighbors,
            ));
    }
}