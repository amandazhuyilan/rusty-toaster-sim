pub struct Frame {
    pub num_actors: usize,
    pub timestamp: usize,
}

pub struct World {
    pub total_actors: usize,
    pub sim_time: usize,
    pub total_frames: usize,
}
