use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::tailwind,
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

const ORB_COLOR: Color = Color::srgb(0.8, 0.1, 0.1);
const ORB_INITIAL_POSITION: Vec3 = Vec3::new(0.0, -50.0, 2.0);
const ORB_INITIAL_VELOCITY: Vec2 = Vec2::new(0.5, -0.5);
const ORB_DIAMETER: f32 = 30.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("setup()");
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ORB_COLOR)),
        Transform::from_translation(ORB_INITIAL_POSITION)
            .with_scale(Vec2::splat(ORB_DIAMETER).extend(1.0)),
        Orb,
        Velocity(ORB_INITIAL_VELOCITY),
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
        transform.translation.x += velocity.x * elapsed;
        transform.translation.y += velocity.y * elapsed;
        //println!("{elapsed} {:?}", transform.translation);
    }
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
        .insert_resource(AngularVelocityZ(0.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (keypress_event, apply_velocity).chain(),
        )
        .run();
}
