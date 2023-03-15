use bevy::prelude::*;
mod gameplay;
mod monsters;
mod player;

fn main() {
    App::new()
        .add_startup_system(setup)
        .add_system(call_npc)
        .run();
}

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
