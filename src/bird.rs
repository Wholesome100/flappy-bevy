use avian2d::{math::*, prelude::*};
use bevy::{color::palettes::tailwind::GREEN_600, prelude::*};

// Needed to check if any colliding entities are Deadly
use crate::obstacles::Deadly;

/// Plugin for the player character controller
pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
        app.add_systems(Update, flap_bird);
    }
}

#[derive(Component)]
pub struct Controllable;

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
    let bird_position = Vec3::new(-26., 0., 0.);

    let bird_matrix = Mat4::from_rotation_translation(bird_orient, bird_position);

    commands
        .spawn((
            Mesh2d(meshes.add(bird_shape)),
            MeshMaterial2d(materials.add(bird_color)),
            RigidBody::Dynamic,
            MaxLinearSpeed(20.0),
            AngularDamping(10.0),
            GravityScale(2.5),
            Collider::from(bird_shape),
            CollisionEventsEnabled,
            Transform::from_matrix(bird_matrix),
            Controllable,
        ))
        .observe(on_deadly_contact);
}

/// Update method to let the bird "flap" on every spacebar press
fn flap_bird(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut birds: Query<Forces, With<Controllable>>,
) {
    for mut forces in &mut birds {
        // Debgugging line to see rotation of bird
        // println!("{:?}", forces.rotation().as_degrees());

        // Apply upwards linear impulse on spacebar press
        if keyboard_input.just_pressed(KeyCode::Space) {
            forces.apply_linear_impulse(Vec2::new(0.0, 1000.0));

            // Apply angular impulse only when under 100.0 degrees to stop spinning out
            if forces.rotation().as_degrees() < 100.0 {
                //println!("Applying positive impulse");
                forces.apply_angular_impulse(1000.0);
            }

            //println!("Flap!")
        }

        // Apply negative angular impulse only while we're above 20.0 degrees
        if forces.rotation().as_degrees() > 20.0 {
            forces.apply_angular_impulse(-5.0);
        }

        // We set forces after the impulse due to borrowing
        let bird_force = forces.linear_velocity_mut();
        bird_force.x = 0.0;
    }
}

fn on_deadly_contact(event: On<CollisionStart>, deadly_query: Query<&Deadly>) {
    let bird = event.collider1;
    let obstacle = event.collider2;

    if deadly_query.contains(obstacle) {
        println!("The {bird} is dead.")
    }
}
