use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Motor {
    pub speed: f32,
    pub direction: Vec3,
}