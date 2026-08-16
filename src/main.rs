use bevy::prelude::*;

mod asset_importer;
mod player;

use crate::asset_importer::AssetImporterPlugin;
use crate::player::PlayerPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AssetImporterPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(2.0, 4.0, 2.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
