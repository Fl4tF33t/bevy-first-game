use bevy::prelude::*;
use super::resources::*;
use super::components::*;

pub fn draw_grid_cells(
    mut gizmos: Gizmos,
    grid: Res<Grid3d>,
){
    let dimensions = grid.dimensions;
    let size = grid.cell_size;
    let color = Color::srgba(1.0, 1.0, 1.0, 0.01);
    for x in 0..dimensions.x {
        for y in 0..dimensions.y {
            for z in 0..dimensions.z {
                let cell = IVec3::new(x as i32, y as i32, z as i32);
                let position = grid.cell_to_world(cell);
                gizmos.cube(
                    Transform::from_translation(position)
                        .with_scale(Vec3::splat(size)),
                    color,
                );
            }
        }
    }
}

pub fn draw_occupied_cells(
    mut gizmos: Gizmos,
    grid: Res<Grid3d>,
    boid_grid: Res<SpatialGrid>,
){
    let size = grid.cell_size * 0.99;
    let color = Color::srgba(0.0, 1.0, 0.0, 0.05);
    for &cell in boid_grid.0.keys(){
        let position = grid.cell_to_world(cell);
        gizmos.cube(
            Transform::from_translation(position)
                .with_scale(Vec3::splat(size)),
            color,
        );
    }
}

pub fn draw_lines_to_neighbors(
    mut gizmos: Gizmos,
    query: Query<(Entity, &Transform, &BoidAgent)>,
    transforms: Query<&Transform, With<BoidAgent>>,
){
    let color = Color::srgba(1.0, 0.0, 0.0, 0.1);
    for (entity, transform, boid) in &query {
        for &neighbor in &boid.neighbors {
            if neighbor.index() <= entity.index() {
                continue;
            }
            if let Ok(neighbor_transform) = transforms.get(neighbor) {
                gizmos.line(
                    transform.translation,
                    neighbor_transform.translation,
                    color,
                );
            }
        }
    }
}

pub fn draw_bounds(
    mut gizmos: Gizmos,
    grid: Res<Grid3d>,
){
    let size = grid.dimensions.as_vec3() * grid.cell_size * 1.001;
    gizmos.cube(
        Transform::from_scale(size),
        Color::srgba(0.0, 0.5, 1.0, 0.5)
    );
}