use crate::utils;

pub enum State {
    NotStarted,
    InProgress,
    Done,
}

pub struct Action {
    pub name: String,
    pub state: State,
    pub start_timestamp: usize,
    pub end_timestamp: usize,
}

pub struct Actor {
    pub id: usize,
    pub name: String,
    pub position: utils::Vector2d,
    pub action: Action,
    pub last_updated_timestamp: usize,
}

pub fn spawn(id: usize,
             name: String, 
             position: utils::Vector2d, 
             action_name: String,
             action_duration: usize,
             current_timestamp: usize) -> Actor {
    // TODO: Figure out who to own the created actor.
    // Ideally it should be owned by the actor array, but we need a way to push the new actor object (or referrence) into the actor array
    Actor {
        id: id,
        name: name,
        position: position,
        action: Action {
            name: action_name,
            // TODO: add feature where action can start certain time after the actor is spawned
            state: State::NotStarted,
            start_timestamp: current_timestamp,
            end_timestamp: current_timestamp + action_duration,
        },
        last_updated_timestamp: current_timestamp,
    }
}