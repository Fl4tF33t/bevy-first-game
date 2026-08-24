use bevy::prelude::*;

#[derive(Component, Default)]
pub struct BoidAgent{
    pub neighbors: Vec<Entity>,
    pub steering: Vec3,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum BoidSet {
    PreSimulation,
    Calculation,
    Simulation,
    PostSimulation,
    Debug,
}