use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BoidAgent{
    pub neighbors: Vec<Entity>,
}
