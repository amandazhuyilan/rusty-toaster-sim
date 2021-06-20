# `Rust`y Toaster Sim
An autonomous vehicle software simulator in Rust, inspired by a toaster simulator idea.

## Build
cargo build

## Test
cargo test

## Terminology
- State
A state represents the dynamic status of an actor, at a specific timestamp.
- Actor
An actor is an object in the sim world. An actor always has and only has one state.

- Event
An event has the potential to trigger state(s) change(s). Not all events change states, but any state changes are caused by event(s). Event can only be applied to actor(s) instead of directly to state(s).

- Frame
A frame is a snapshot of what's happening in the sim world labelled with a timestamp. A frame contains reference to all states of the actors in the sim world.

## Checklist
- [x] Set up cargo project and define core terminologies
- [x] Set up one stationary `actor` and output name of actor
- [ ] Implement "world" feature where frame data at a specific timestamp can be made available when queried
- [ ] Implement "clock" feature where the new frames are generated when "clock" is ticked
- [ ] Set up one dynamic `actor`. This actor should be able to move around the sim world (status printed to `stdout`)
- [ ] Set up 2 dynamic `actor`s that will collide with each other