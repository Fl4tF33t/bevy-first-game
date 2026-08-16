use bevy::prelude::*;
use super::resources::*;

pub fn import_assets(
    app: &mut App,
) {
    let asset_server = app.world().resource::<AssetServer>();

    let player_scene = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("Cat_1.glb")
    );

    app.insert_resource(AssetImporter {
        player_scene,        
    });
}