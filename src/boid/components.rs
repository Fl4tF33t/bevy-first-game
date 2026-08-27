use bevy::prelude::*;
use rand::*;
use super::resources::*;

#[derive(Component, Default, Debug)]
pub struct BoidAgent{
    pub neighbors: Vec<Entity>,
    pub separation: Vec3,
    pub cohesion: Vec3,
    pub alignment: Vec3,
    pub bound: Vec3,
    pub velocity: Vec3
}

#[derive(Clone, Copy)]
pub struct BoidSnapShot {
    pub entity: Entity,
    pub position: Vec3,
    pub velocity: Vec3
}

#[derive(Component, Default)]
pub struct StoredVelocity(pub Vec3);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum BoidSet {
    PreSimulation,
    Calculation,
    Simulation,
    PostSimulation,
    Debug,
}

pub trait RandomVec3 {
    fn random_position() -> Self;
    fn random_velocity() -> Self;
}

impl RandomVec3 for Vec3 {
    fn random_position() -> Self {
        let mut rng = rng();
        let limit = bounds_limit();
        let x  = rng.random_range(-limit..limit);
        let y  = rng.random_range(-limit..limit);
        let z  = rng.random_range(-limit..limit);
        Vec3::new(x, y, z)
    }
    fn random_velocity() -> Self {
        let mut rng = rng();
        let x  = rng.random_range(-1.0..1.0);
        let y  = rng.random_range(-1.0..1.0);
        let z  = rng.random_range(-1.0..1.0);
        Vec3::new(x, y, z)
    }
}