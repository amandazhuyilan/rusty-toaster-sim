use bevy::prelude::*;
use heron::prelude::*;
use bevy::core::FixedTimestep;

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;   // 1 frame per second

struct EgoCar;
struct State {
    speed: f32, 
}

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Define gravity
        .add_startup_system(spawn.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(update_ego_car_position.system())
        )
        // .add_system(update_ego_car_position.system()) 
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
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
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
        .insert(EgoCar);
}

fn update_ego_car_position(
    mut ego_car_position: Query<&mut Transform, With<EgoCar>>,
) {
    // iterate through all entities that has the Actor and transform component 
    for mut transform in ego_car_position.iter_mut() {
        transform.translation.x += 2.;
    }
}