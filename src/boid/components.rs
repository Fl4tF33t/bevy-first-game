use bevy::prelude::*;
use rand::*;

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

pub trait RandomVec3 {
    fn random() -> Self;
}

impl RandomVec3 for Vec3 {
    fn random() -> Self {
        let mut rng = rng();
        let (x, y, z): (f32, f32, f32) = rng.random();

        Vec3::new(x, y, z)
    }
}