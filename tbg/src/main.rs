use bevy::prelude::*;
use bevy::window::PrimaryWindow;
mod gameplay;
mod monsters;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(player_movment)
        .run();
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/playerShip1_red.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub const PLAYER_SPEED: f32 = 200.0;

pub fn player_movment(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    //Create mut tranform obejct that has direction value
    //remember use get_single or iter with _mut when want to change data
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
/*
fn call_npc(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((Person, Name("Sonic".to_string())));
    commands.spawn((Person, Name("Mario".to_string())));
    commands.spawn((Person, Name("Link".to_string())));
}

//Breaked entity components
//Needs to be pub for Query?
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
*/

/*
#[derive(Component)]
struct Health(u8)
#[derive(Component)]
struct Attack(u8)
#[derive(Bundle)]
struct Npc {
    health: Health,
    attack: Attack,
}
*/
