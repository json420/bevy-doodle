use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::tailwind,
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

const ORB_COLOR: Color = Color::srgb(0.8, 0.1, 0.1);
const ORB_INITIAL_POSITION: Vec3 = Vec3::new(0.0, -50.0, 2.0);
const ORB_INITIAL_VELOCITY: Vec2 = Vec2::new(25.0, -40.0);
const ORB_DIAMETER: f32 = 30.0;
const ORB_SPEED: f32 = 50.0;

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
struct CurrentVelocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Orb;

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
    current_velocity: Res<CurrentVelocity>,
) {
    for (mut transform, mut velocity) in &mut query {
        let elapsed = time.delta_secs();
        transform.translation.x += velocity.x * elapsed;
        transform.translation.y += velocity.y * elapsed;
        //println!("{elapsed} {:?}", transform.translation);
    }
}

fn keypress_event(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_velocity: ResMut<CurrentVelocity>,
) {
    let mut vx = 0.0;
    let mut vy = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        vx -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        vx += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        vy -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        vy += 1.0;
    }

    current_velocity.0 = Vec2::new(vx, vy).normalize() * ORB_SPEED;
    if vx != 0.0 || vy != 0.0 {
        println!("{:?}", current_velocity.0);
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
        .insert_resource(CurrentVelocity(Vec2::new(0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (keypress_event, apply_velocity).chain())
        .run();
}
