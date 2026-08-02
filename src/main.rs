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
const ORB_SPEED: f32 = 150.0;

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
        transform.translation.x += current_velocity.x * elapsed;
        transform.translation.y += current_velocity.y * elapsed;
    }
}

fn keypress_event(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_velocity: ResMut<CurrentVelocity>,
) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        dx -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dx += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        dy += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dy -= 1.0;
    }

    current_velocity.0 = if dx != 0.0 || dy != 0.0 {
        Vec2::new(dx, dy).normalize() * ORB_SPEED
    } else {
        Vec2::new(0.0, 0.0)
    };
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
