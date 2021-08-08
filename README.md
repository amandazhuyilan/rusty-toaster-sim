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
- State
A state represents the dynamic status of an actor, at a specific timestamp.
- Actor
An actor is an object in the sim world. An actor always has and only has one state.

- Event
An event has the potential to trigger state(s) change(s). Not all events change states, but any state changes are caused by event(s). Event can only be applied to actor(s) instead of directly to state(s).

- Frame
A frame is a snapshot of what's happening in the sim world labelled with a timestamp. A frame contains reference to all states of the actors in the sim world.

## `TODO` Checklist
- [x] Set up cargo project and define core terminologies
- [x] Select and integrate base game engine and physics engine
- [x] Update Ego car pose (move some distance per fixed timestep) based on fixed time step.
- [ ] Add `Velocity` as component of Ego car and update Ego car position based on pre-set velocity
- [ ] Method to set Ego car start and destination, periodically check if Ego car reaches destination, quit when it does.
- [ ] Implement `GetWorldState()` method
