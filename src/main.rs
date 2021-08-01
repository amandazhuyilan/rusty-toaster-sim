use bevy::prelude::*;
use heron::prelude::*;

struct Actor;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Define gravity
        .add_startup_system(spawn.system())
        .add_system(actor_movement.system()) 
        .run();
}

fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add camera for visualization
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // the size of our sprite in pixels
    let size = Vec2::new(30.0, 30.0);
    commands
        //  here we add a Sprite. We can add any bundle of our choice; the
        // only required component is a GlobalTransform
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(size),
            material: materials.add(Color::GREEN.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        // Make it a physics body, by adding the RigidBody component
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Cuboid {
            // let the size be consistent with our sprite
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        })
        .insert(Actor);
}

fn actor_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // the `With` type allows us to say “I want entities that have a snake head,
    // but I don’t care about the snake head component itself, just give me the transform”. 
    mut actor_positions: Query<&mut Transform, With<Actor>>,
) {
    // iterate through all entities that has the Actor and transform component 
    for mut transform in actor_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
    }
}