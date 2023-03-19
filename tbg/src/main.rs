use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
// mod gameplay;
// mod monsters;
// mod player;

pub const PLAYER_SPEED: f32 = 300.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: u8 = 5;
pub const ENEMY_SIZE: f32 = 64.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_system(player_movment)
        .add_system(enemy_movement)
        .add_system(enemy_direction)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

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

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window_area = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window_area.width();
        let random_y = random::<f32>() * window_area.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/enemyBlack1.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        //Vec2 can't use += operator what sucks little bit. Should check why it is this way
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
        }
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
