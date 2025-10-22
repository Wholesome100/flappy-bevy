// To build in debug with dynamic linking, run:
// cargo run --features bevy/dynamic_linking
use avian2d::{math::*, prelude::*};
use bevy::{camera::ScalingMode, color::palettes::tailwind::GREEN_600, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(0.25),
        ))
        .add_systems(Startup, (spawn_bird, setup_camera))
        .add_systems(Update, flap_bird)
        .run();
}

#[derive(Component)]
struct Controllable;

/// Startup method to spawn the bird (player character)
fn spawn_bird(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // It's better to declare all meshes and materials as separate variables to make the commands.spawn cleaner
    let bird_shape = Rectangle::new(5.0, 5.0);
    let bird_color = ColorMaterial::from_color(GREEN_600);

    commands.spawn((
        Mesh2d(meshes.add(bird_shape)),
        MeshMaterial2d(materials.add(bird_color)),
        RigidBody::Dynamic,
        Collider::from(bird_shape),
        GravityScale(2.0),
        Controllable,
    ));
}

/// Update method to let the bird "flap" on every spacebar press
fn flap_bird(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut birds: Query<(&mut LinearVelocity, &mut AngularVelocity), With<Controllable>>,
) {
    let delta_time = time.delta_secs_f64().adjust_precision();

    for (mut linear_velocity, mut angular_velocity) in &mut birds {
        if keyboard_input.just_pressed(KeyCode::Space) {
            linear_velocity.y = 1500.0 * delta_time;
            angular_velocity.0 = 50.0 * delta_time;

            println!("Flap!")
        }
    }
}

/// Startup method to initialize the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.04,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
