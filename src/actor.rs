use crate::utils;

pub struct State {
    pub position: utils::Vector2d,
    pub speed: f64,
    pub last_updated_timestamp: u64,
}

pub struct Actor {
    pub id: u32,
    pub name: String,
    pub state: State,
}