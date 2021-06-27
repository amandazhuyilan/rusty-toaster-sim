mod actor;
mod event;
mod world;
mod utils;

use std::time::Duration;
use std::thread::sleep;

fn init_sim_world() -> Result<i32, &'static String> {
    // set up a clean world
    let mut all_actors: Vec::<actor::Actor> = Vec::<actor::Actor>::new();
    let mut sim_time_now: usize = 0;

    let actor1 = actor::spawn(
        all_actors.len(),
        String::from("shiba_0"),
        utils::Vector2d(0.0, 0.0), 
        String::from("eat burrito"),
        10, sim_time_now);
    
    all_actors.push(actor1);
    Ok(0)
}

fn main() {
    println!("Sim world initializing!");
    match init_sim_world() {
        Ok(_) => println!("Sim world ready!"),
        Err(e) => println!("Problem starting sim world: {}", e),
    };

    // the for loop should be checking for end of simulation flag instead
    for current_timestep in 0 .. 10 {
        println!("Sim world running - current time step is: {:?}", current_timestep);
        sleep(Duration::new(1, 0));
    }
    println!("Simulation over!");
    println!("Sim world destroyed");
}