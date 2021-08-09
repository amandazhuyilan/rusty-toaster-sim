# `Rust`y Toaster Sim
A simulator for a (smart) toaster on wheels, in Rust, powered by [Bevy](https://bevyengine.org/).

## Run
```shell
cargo run
```

## Test
```shell
cargo test
```

## Clean
```shell
cargo clean
```

## Terminology
- Actor

An actor is an object in the sim world. An actor always has and only has one state.

- Ego Car

The toaster on wheels, or formally, the system to simulate. Note that the ego car is an actor in the sim world.


## `TODO`
- [x] Set up cargo project and define core terminologies
- [x] Select and integrate base game engine and physics engine
- [x] Update Ego car pose (move some distance per fixed timestep) based on fixed time step.
- [x] Add `Velocity` as component of Ego car and update Ego car position based on pre-set velocity
- [x] Set Ego and actor cars start/destination, periodically check if cars reached destination, update transform of the car if not reached destination.
- [ ] Add heading for Ego and Actor cars
- [ ] Use physic engine methods to check for collision
