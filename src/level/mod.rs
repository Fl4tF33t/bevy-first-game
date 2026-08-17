use bevy::prelude::*;
use systems::*;

mod systems;
pub mod components;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_level);
    }
}