use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Motor {
    pub speed: f32,
    pub direction: Vec3,
    pub vertical_velocity: f32,
    pub grounded: bool,
}
