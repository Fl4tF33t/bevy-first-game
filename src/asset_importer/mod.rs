use bevy::prelude::*;
use systems::*;

pub mod resources;
mod systems;

pub struct AssetImporterPlugin;

impl Plugin for AssetImporterPlugin {
    fn build(&self, app: &mut App) {
        import_assets(app);
    }
}