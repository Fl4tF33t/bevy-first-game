use bevy::prelude::*;
use avian3d::prelude::*;

mod asset_importer;
mod player;
mod level;

use crate::asset_importer::AssetImporterPlugin;
use crate::player::PlayerPlugin;
use crate::level::LevelPlugin;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_plugins(AssetImporterPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(LevelPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(
            0.0, 14.0, -12.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
