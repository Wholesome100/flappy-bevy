use avian2d::{math::*, prelude::*};
use bevy::{color::palettes::tailwind::GREEN_600, prelude::*};

/// Plugin for the player character controller
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
        app.add_systems(Update, flap_bird);
    }
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
    let bird_shape = Capsule2d::new(2.0, 2.0);
    let bird_color = ColorMaterial::from_color(GREEN_600);

    let bird_orient = Quat::from_rotation_z(90.0 * (PI / 180.0));
    let bird_position = Vec3::from_array([-26., 0., 0.]);

    let bird_matrix = Mat4::from_rotation_translation(bird_orient, bird_position);

    commands.spawn((
        Mesh2d(meshes.add(bird_shape)),
        MeshMaterial2d(materials.add(bird_color)),
        RigidBody::Dynamic,
        Collider::from(bird_shape),
        Transform::from_matrix(bird_matrix),
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
        // Keep the linear velocity at 0.0 to keep the bird in one spot
        linear_velocity.x = 0.0;

        // Apply upwards linear velocity and angular velocity on spacebar press
        if keyboard_input.just_pressed(KeyCode::Space) {
            linear_velocity.y = 1500.0 * delta_time;

            angular_velocity.0 = 50.0 * delta_time;

            println!("Flap!")
        }
    }
}
