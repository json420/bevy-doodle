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

const ORB_COLOR: Color = Color::srgb(0.8, 0.1, 0.1);
const ORB_INITIAL_POSITION: Vec3 = Vec3::new(0.0, -50.0, 2.0);
const ORB_INITIAL_VELOCITY: Vec2 = Vec2::new(0.5, -0.5);
const ORB_DIAMETER: f32 = 30.0;

fn spawn_orb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("spawn_orb");
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ORB_COLOR)),
        Transform::from_translation(ORB_INITIAL_POSITION)
            .with_scale(Vec2::splat(ORB_DIAMETER).extend(1.0)),
        Orb,
        Velocity(ORB_INITIAL_VELOCITY),
    ));
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

#[derive(Resource, Deref, DerefMut)]
struct AngularVelocityZ(f32);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Orb;

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        let elapsed = time.delta_secs();
        println!("{elapsed}");
        transform.translation.x += velocity.x * elapsed;
        transform.translation.y += velocity.y * elapsed;
    }
}

// So how does App.add_systems() know *which* sprite to call rotate_sensei() with?
fn rotate_sensei(
    time: Res<Time>,
    mut sprite: Single<&mut Transform, With<Sprite>>,
    avz: Res<AngularVelocityZ>,
) {
    sprite.rotation *= Quat::from_rotation_z(time.delta_secs() * avz.0);
    sprite.translation.x = avz.0 * 10.0;
    //sprite.translation = Vec3::new(0.0, 0.0, 0.0);
    //println!("{:?}", sprite.translation);
}

fn keypress_event(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut avz: ResMut<AngularVelocityZ>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        println!("1");
        avz.0 = 1.2;
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        println!("2");
        avz.0 = 0.5;
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        println!("3");
        avz.0 = 0.0;
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        println!("4");
        avz.0 = -0.5;
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        println!("5");
        avz.0 = -1.2;
    }
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
        .insert_resource(AngularVelocityZ(0.0))
        .add_systems(Startup, setup_sensei)
        .add_systems(Startup, spawn_orb)
        .add_systems(
            Update,
            (keypress_event, rotate_sensei, apply_velocity).chain(),
        )
        .run();
}
