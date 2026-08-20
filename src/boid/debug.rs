use bevy::prelude::*;
use super::resources::*;
use super::components::*;

pub fn draw_grid_cells(
    mut gizmos: Gizmos,
    grid: Res<Grid3d>,
) {
    for x in 0..grid.dimensions.x {
        for y in 0..grid.dimensions.y {
            for z in 0..grid.dimensions.z {
                let cell = IVec3::new(x as i32, y as i32, z as i32);
                let position = grid.cell_to_world(cell);
                gizmos.cube(
                    Transform::from_translation(position)
                        .with_scale(Vec3::splat(grid.cell_size)),
                    Color::srgba(1.0, 1.0, 1.0, 0.01),
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
    for &cell in boid_grid.cells.keys(){
        let position = grid.cell_to_world(cell);
        gizmos.cube(
            Transform::from_translation(position)
                .with_scale(Vec3::splat(grid.cell_size * 0.99)),
            Color::srgba(0.0, 1.0, 0.0, 0.05),
        );
    }
}

pub fn draw_lines_to_neighbors(
    mut gizmos: Gizmos,
    query: Query<(Entity, &Transform, &BoidAgent)>,
    transforms: Query<&Transform, With<BoidAgent>>,
) {
    for (entity, transform, boid) in &query {
        for &neighbor in &boid.neighbors {
            if neighbor.index() <= entity.index() {
                 continue;
            }
            if let Ok(neighbor_transform) = transforms.get(neighbor) {
                gizmos.line(
                    transform.translation,
                    neighbor_transform.translation,
                    Color::srgba(1.0, 0.0, 0.0, 0.1),
                );
            }
        }
    }
}