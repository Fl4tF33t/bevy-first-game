use bevy::prelude::*;

#[derive(Resource)]
pub struct AssetImporter{
    pub player_scene: Handle<WorldAsset>,
}