mod actor;
mod event;
mod frame;
mod world;
mod utils;

fn main() {
    println!("Sim start!");
    println!("Sim world initializing!");
    let state1 = actor::State {
        position: utils::Vector2d(0.0, 0.0),
        speed: 1.0,
        last_updated_timestamp: 1,
    };
    let actor1 = actor::Actor {
        id: 0,
        name: String::from("Tyler"),
        state: state1,
    };

    println!("Sim world ready!");

    println!("Actor with name: {:?} created!", actor1.name);
    // the for loop should be checking for end of simulation flag instead
    for current_timestep in 0 .. 10 {
        println!("Sim world running - current time step is: {:?}.", current_timestep);
    }
    println!("Simulation over!");
    println!("Sim world destroyed");
}