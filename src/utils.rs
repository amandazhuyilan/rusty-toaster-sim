use crate::actor;

pub struct Vector2d(pub f64, pub f64);
pub struct Frame {
    pub actors: Vec<actor::Actor>,
    pub timestamp: u64,
}