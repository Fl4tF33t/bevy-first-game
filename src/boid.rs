mod components;
mod systems;
mod resources;
mod debug;

use bevy::{math::USizeVec3, prelude::*};
use resources::*;
use systems::*;
use debug::*;

use crate::boid::components::BoidSet;

pub struct BoidPlugin;
impl Plugin for BoidPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BoidSettings::new(12, Vec3::ONE, 0.5))
            .insert_resource(Grid3d::new(10.0, USizeVec3::ONE * 10))
            .insert_resource(SpatialGrid::default())

            .configure_sets(Update, (
                BoidSet::PreSimulation,
                BoidSet::Calculation,
                BoidSet::Simulation,
                BoidSet::PostSimulation,
            ).chain())

            .add_systems(Startup, (
                clear_flock,
                spawn_flock,
            ).chain())

            .add_systems(Update, (
                update_boid_grid,
                update_neighbors,
            ).in_set(BoidSet::PreSimulation))

            // .add_systems(Update, (
            //     compute_separation,
            //     compute_alignment,
            //     compute_cohesion,
            // ).in_set(BoidSet::Calculation))

            // .add_systems(Update, (
            //     compute_steering
            //     // velocity
            // ).in_set(BoidSet::Simulation))

            .add_systems(Update, (
                rotate_boid,
            ).in_set(BoidSet::PostSimulation))
            
            .add_systems(Update, (
                draw_grid_cells,
                draw_lines_to_neighbors,
                draw_occupied_cells,
            ).in_set(BoidSet::Debug));
    }
}