use bevy::prelude::*;

struct Actor;
struct Materials {
    head_material: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(1.0, 1.0, 1.0).into())
    });
}

fn spawn_snake(mut commands: Commands, materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(20.0, 20.0)),
        ..Default::default()
    }).insert(Actor);
}

fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    // the `With` type allows us to say “I want entities that have a snake head,
    // but I don’t care about the snake head component itself, just give me the transform”. 
    mut head_positions: Query<&mut Transform, With<Actor>>,
) {
    // iterate through all entities that has the Actor and transform component 
    for mut transform in head_positions.iter_mut() {
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

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
        .add_system(snake_movement.system()) 
        .add_plugins(DefaultPlugins)
        .run();
}