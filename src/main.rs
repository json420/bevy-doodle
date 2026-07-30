use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::tailwind,
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

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

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}

fn setup_sensei(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup");
    commands.spawn((Camera2d, IsDefaultUiCamera));
    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));

    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(tailwind::ROSE_400.into()),
        children![(
            Node {
                height: percent(30),
                width: percent(20),
                min_height: px(150),
                min_width: px(150),
                border: UiRect::all(px(2)),
                border_radius: BorderRadius::all(percent(25)),
                ..Default::default()
            },
            BorderColor::all(Color::WHITE),
        )],
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("textures/rpg/chars/sensei/sensei.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        RenderLayers::layer(1),
    ));
}

fn rotate_sensei(time: Res<Time>, mut sprite: Single<&mut Transform, With<Sprite>>) {
    sprite.rotation *=
        Quat::from_rotation_z(time.delta_secs() * 0.5) * Quat::from_rotation_y(time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(HelloPlugin)
        .add_systems(Startup, setup_sensei)
        .add_systems(Update, rotate_sensei)
        .run();
}
