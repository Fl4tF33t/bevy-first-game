use bevy::prelude::*;
use avian3d::prelude::*;
use super::components::*;

pub fn create_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Level{},
        Transform::from_xyz(0.0, 0.0, 0.0),

        // Visible floor
        Mesh3d(meshes.add(
            Plane3d::default().mesh().size(100.0, 100.0)
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.2, 0.3),
            ..default()
        })),

        // Infinite physics floor
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));
}