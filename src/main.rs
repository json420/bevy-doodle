use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
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

fn main() {
    App::new().add_systems(Update, hello_world).run();
}
