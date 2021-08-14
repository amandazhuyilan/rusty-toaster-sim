use bevy::prelude::*;
use heron::prelude::*;
use bevy::core::FixedTimestep;

const TIMESTEP_1_PER_SECOND: f64 = 60.0 / 60.0;   // 1 frame per second

struct EgoCar;
struct ActorCar;
struct EgoStateText;

struct ActorState {
    velocity: Vec3, // pixel per second
    start: Vec3,    // starting position
    end: Vec3,     // ending position
}

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default()) // Add the Heron plugin
        .insert_resource(Gravity::from(Vec3::new(0.0, -300.0, 0.0))) // Define gravity
        .add_startup_system(setup_scene.system())
        .add_startup_system(spawn_actors.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                .with_system(update_actors_transforms.system())
        )
        .add_system(text_update_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    // Rich text with multiple sections
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Ego Vehicle Position: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EgoStateText);
}

fn spawn_actors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // Add camera for visualization
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // the size of our sprite (pixels)
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
        .insert(ActorState {
            velocity: Vec3::new(2.0, 0.0, 0.0),
            // TODO: start should be the same as when setting up the sprite bundle
            start: Vec3::new(5.0, 0.0, 0.0),
            end: Vec3::new(20.0, 0.0, 0.0),
        })
        .insert(EgoCar);

    commands
        // here we add a Sprite. We can add any bundle of our choice; the
        // only required component is a GlobalTransform
        .spawn_bundle(SpriteBundle {
            // the size of our sprite in pixels
            sprite: Sprite::new(size),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_translation(Vec3::new(100.0, 0.0, 0.0)),
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
        .insert(ActorState {
            velocity: Vec3::new(4.0, 0.0, 0.0),
            // TODO: start should be the same as when setting up the sprite bundle
            start: Vec3::new(100.0, 0.0, 0.0),
            end: Vec3::new(150.0, 0.0, 0.0),
        })
        .insert(ActorCar);
}

fn update_actors_transforms(mut query: Query<(&ActorState, &mut Transform)>) {
    // iterate through all entities that has the Actor and transform component 
    for (actor_state, mut transform) in query.iter_mut() {
        if transform.translation != actor_state.end {
            transform.translation.x += actor_state.velocity.x;
        }
    }
}

fn text_update_system(
    mut actor_query: Query<&Transform, With<EgoCar>>,
    mut text_query: Query<&mut Text, With<EgoStateText>>) {
        for transform in actor_query.iter_mut() {
            for mut text in text_query.iter_mut() {
                text.sections[1].value = format!("{:.2}, {:.2}, {:.2}", transform.translation.x, transform.translation.y, transform.translation.z);
            }
        }
}