use crate::{SceneItemId, ScenePoint};

pub mod item;
pub mod path;
pub mod util;
pub mod visibility;

#[derive(Debug)]
pub enum PhysicEvent {
    Explosion,
    BulletFire(ScenePoint, ScenePoint, Option<SceneItemId>, HitType),
}

#[derive(Debug)]
pub enum MetaEvent {
    FeelExplosion,
}

pub enum HitType {
    Deadly,
    Incapacity,
    Hurting,
    VeryClose,
    Missed,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32,
}

impl GridPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
