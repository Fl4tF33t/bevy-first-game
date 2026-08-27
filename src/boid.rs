mod components;
mod systems;
mod resources;
mod debug;

use bevy::prelude::*;
use resources::*;
use systems::*;
use debug::*;

use crate::boid::components::BoidSet;

pub struct BoidPlugin;
impl Plugin for BoidPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BoidSettings::new())
            .insert_resource(Grid3d::new())
            .insert_resource(SpatialGrid::default())

            .configure_sets(Update, (
                BoidSet::PreSimulation,
                BoidSet::Calculation,
                BoidSet::Simulation,
                BoidSet::PostSimulation
            ).chain())

            .add_systems(Startup, (
                clear_flock,
                spawn_flock,
                rotate_flock,
            ).chain())

            .add_systems(FixedUpdate, (
                update_boid_grid,
                update_neighbors,
            ).in_set(BoidSet::PreSimulation))

            .add_systems(FixedUpdate, (
                compute_separation,
                compute_alignment,
                compute_cohesion,
                compute_bounds
            ).in_set(BoidSet::Calculation))

            .add_systems(FixedUpdate, (
                compute_steering
                // velocity
            ).in_set(BoidSet::Simulation))

            // .add_systems(FixedUpdate, (
            //     reset_steering,
            //     // velocity
            // ).in_set(BoidSet::PostSimulation))

            // .add_systems(Update, (
            //     rotate_boid,
            // ))
            
            .add_systems(Update, (
                draw_grid_cells,
                draw_lines_to_neighbors,
                draw_occupied_cells,
                draw_bounds,
            ).in_set(BoidSet::Debug));
    }
}