use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Joan".to_string())));
    commands.spawn((Person, Name("Tina".to_string())));
    commands.spawn((Person, Name("Fred".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Tina" {
            name.0 = "Sue".to_string();
            break;
        }
    }
}

struct Entity(u64);

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

fn hello_world() {
    println!("hello, world.");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (
                hello_world,
                (greet_people, update_people, greet_people).chain(),
            ),
        )
        .run();
}
