use bevy::{
    prelude::*,
    window::{
        PresentMode, PrimaryWindow, VideoModeSelection, WindowMode, WindowPlugin, WindowResized,
    },
};

const ORB_COLOR: Color = Color::srgb(0.9, 0.1, 0.4);
const ORB_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 2.0);
const ORB_RADIUS: f32 = 15.0;
const ORB_DIAMETER: f32 = ORB_RADIUS * 2.0;
const ORB_MAX_SPEED: f32 = 1800.0;
const ORB_ACCELERATION: f32 = 999.0;
const ORB_DRAG_FACTOR: f32 = 0.3;

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("setup()");
    commands.spawn(Camera2d);
    commands.spawn((
        Orb,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(ORB_COLOR)),
        Transform::from_translation(ORB_INITIAL_POSITION)
            .with_scale(Vec2::splat(ORB_DIAMETER).extend(1.0)),
    ));
    let sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(sound));
}

#[derive(Event)]
struct OrbCollided;

fn play_sound(_collided: On<OrbCollided>, mut commands: Commands, sound: Res<CollisionSound>) {
    commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
}

#[derive(Resource, Deref, DerefMut)]
struct State {
    #[deref]
    acceleration: Vec2,
    velocity: Vec2,
    bottom_left: Vec2,
    top_right: Vec2,
}

#[derive(Component)]
struct Orb;

fn apply_velocity(
    mut query: Query<(&mut Transform, &mut MeshMaterial2d<ColorMaterial>)>,
    time: Res<Time>,
    mut state: ResMut<State>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut transform, mut material_2d) in &mut query {
        let elapsed = time.delta_secs();
        transform.translation.x += state.velocity.x * elapsed;
        transform.translation.y += state.velocity.y * elapsed;
        let s = Vec2::new(transform.translation.x, transform.translation.y);
        println!("translation: {}", s);
        let mut collided = false;
        if transform.translation.x < state.bottom_left.x
            || transform.translation.x > state.top_right.x
        {
            state.velocity.x *= -1.0;
            collided = true;
        }
        if transform.translation.y < state.bottom_left.y
            || transform.translation.y > state.top_right.y
        {
            state.velocity.y *= -1.0;
            collided = true;
        }
        transform.translation = s.clamp(state.bottom_left, state.top_right).extend(2.0);
        if collided {
            println!("collided");
            commands.trigger(OrbCollided);
            if let Some(mut m) = materials.get_mut(&material_2d.0) {
                m.color = Color::srgb(0.0, 1.0, 0.0);
                println!("color");
            }
        }
    }
}

fn keypress_event(keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut state: ResMut<State>) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        dx -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        dx += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        dy += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        dy -= 1.0;
    }

    state.acceleration = if dx != 0.0 || dy != 0.0 {
        Vec2::new(dx, dy).normalize() * ORB_ACCELERATION
    } else {
        Vec2::ZERO
    } - state.velocity * ORB_DRAG_FACTOR;
    state.velocity = apply_acceleration(state.acceleration, state.velocity, time.delta_secs());
}

fn on_resize_system(mut resize_reader: MessageReader<WindowResized>, mut state: ResMut<State>) {
    for m in resize_reader.read() {
        println!("resize: {:?}", m);
        let x = m.width / 2.0;
        let y = m.height / 2.0;
        state.bottom_left = Vec2::new(-x + ORB_RADIUS, -y + ORB_RADIUS);
        state.top_right = Vec2::new(x - ORB_RADIUS, y - ORB_RADIUS);
    }
}

fn toggle_fullscreen(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = window_query.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => {
                    WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current)
                }
                _ => WindowMode::Windowed,
            };
        }
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
        .insert_resource(State {
            acceleration: Vec2::new(0.0, 0.0),
            velocity: Vec2::ZERO,
            bottom_left: Vec2::NEG_ONE,
            top_right: Vec2::ONE,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_fullscreen,
                on_resize_system,
                keypress_event,
                apply_velocity,
            )
                .chain(),
        )
        .add_observer(play_sound)
        .run();
}

fn clamp_velocity(velocity: Vec2) -> Vec2 {
    if velocity.length_squared() < 5.0 {
        Vec2::new(0.0, 0.0)
    } else if velocity.length() > ORB_MAX_SPEED {
        velocity.normalize() * ORB_MAX_SPEED
    } else {
        velocity
    }
}

fn apply_acceleration(acceleration: Vec2, velocity: Vec2, elapsed: f32) -> Vec2 {
    let delta = acceleration * elapsed;
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
