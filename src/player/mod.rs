use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, entity_motor)
            .add_systems(Update, read_movement_inputs)
            .add_systems(Update, apply_kinematic_gravity);
    }
}