use bevy::{
    prelude::*,
    window::{PresentMode, WindowPlugin},
};

const ORB_COLOR: Color = Color::srgb(0.9, 0.1, 0.4);
const ORB_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 2.0);
const ORB_INITIAL_VELOCITY: Vec2 = Vec2::new(25.0, -40.0);
const ORB_DIAMETER: f32 = 30.0;
const ORB_SPEED: f32 = 150.0;
const ORB_MAX_SPEED: f32 = 300.0;
const ORB_ACCELERATION: f32 = 150.0;
const ORB_DRAG: f32 = 200.0;

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
struct State {
    #[deref]
    acceleration: Vec2,
    velocity: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Orb;

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
    state: Res<State>,
) {
    for (mut transform, mut velocity) in &mut query {
        let elapsed = time.delta_secs();
        transform.translation.x += state.velocity.x * elapsed;
        transform.translation.y += state.velocity.y * elapsed;
    }
}

fn keypress_event(keyboard: Res<ButtonInput<KeyCode>>, mut state: ResMut<State>) {
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

    state.acceleration = if dx != 0.0 || dy != 0.0 {
        Vec2::new(dx, dy).normalize() * ORB_ACCELERATION
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
        .insert_resource(State {
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (keypress_event, apply_velocity).chain())
        .run();
}

fn clamp_velocity(velocity: Vec2) -> Vec2 {
    if velocity.length() > ORB_MAX_SPEED {
        velocity.normalize() * ORB_MAX_SPEED
    } else {
        velocity
    }
}

fn compute_drag(velocity: Vec2) -> Vec2 {
    velocity.normalize() * -ORB_DRAG
}

fn apply_acceleration(acceleration: Vec2, velocity: Vec2, elapsed: f32) -> Vec2 {
    let drag = compute_drag(velocity);
    let delta = (acceleration + drag) * elapsed;
    clamp_velocity(velocity + delta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_velocity() {
        assert_eq!(clamp_velocity(Vec2::new(1.0, 2.0)), Vec2::new(1.0, 2.0));
        assert_eq!(
            clamp_velocity(Vec2::new(-ORB_MAX_SPEED, ORB_MAX_SPEED)),
            Vec2::new(-1.0, 1.0).normalize() * ORB_MAX_SPEED
        );
    }
}
