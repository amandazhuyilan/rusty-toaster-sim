// a simple bevy app that prints name of actors per 2 seconds.
// reference from the bevy book: https://bevyengine.org/learn/book/getting-started/
use bevy::prelude::*;

struct Name(String);
struct Person;
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Tyler1".to_string()));
    commands.spawn().insert(Person).insert(Name("Tyler2".to_string()));
    commands.spawn().insert(Person).insert(Name("Tyler3".to_string()));
}

fn greet_people(time:Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}", name.0);
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}