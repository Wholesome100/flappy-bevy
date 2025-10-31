use std::time::Duration;

use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::{PURPLE_400, RED_800},
    prelude::*,
};
use rand::Rng;

use crate::bird::Controllable;

/// Plugin for the obstacles the player will interact with, including ground/sky
pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Startup, spawn_borders)
            .add_systems(FixedUpdate, (spawn_pipes, move_pipes, despawn_pipes).chain());
    }
}

#[derive(Resource)]
struct PipeTimer(Timer);

#[derive(Component)]
pub struct Deadly;

#[derive(Component)]
pub struct Moving;

fn spawn_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let border_shape = Rectangle::new(200.0, 10.0);
    let border_color = ColorMaterial::from_color(RED_800);

    // Entity block for the ground
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color.clone())),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., -45., 0.),
        Deadly,
    ));

    // Entity block for the sky
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color)),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., 55., 0.),
    ));
}

fn spawn_pipes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut spawn_time: ResMut<PipeTimer>,
) {
    // To make a note of what else needs to be done:
    // 1. Pipes need to spawn at random time intervals
    const PIPE_TIME: (f32, f32) = (4.0, 8.0);
    let mut t_rng = rand::rng();

    // 2. Pipe shapes need to vary with the sensor gap

    // y_min is -40, ymax is 50
    let pipe_shape = Rectangle::new(10., 80.);
    let pipe_color = ColorMaterial::from_color(PURPLE_400);

    // Offset accounting for borders
    const OFFSET: f32 = 5.0;

    // Offscreen spawn value for pipes
    const OFFSCREEN_X: f32 = 100.0;

    spawn_time.0.tick(time.delta());

    if spawn_time.0.is_finished() {
        commands
            .spawn((Visibility::default(), RigidBody::Kinematic, Moving))
            .with_children(|parent| {
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(10., 45.))),
                    MeshMaterial2d(materials.add(pipe_color)),
                    Collider::from(Rectangle::new(10., 45.)),
                    Transform::from_xyz(OFFSCREEN_X, 0.0 + OFFSET - 22.5, 0.),
                    Deadly,
                ));

                parent
                    .spawn((
                        Collider::from(pipe_shape),
                        Transform::from_xyz(OFFSCREEN_X, 0.0 + OFFSET, 0.),
                        Sensor,
                        CollisionEventsEnabled,
                    ))
                    .observe(award_point);
            });

        let bingus = t_rng.random_range(PIPE_TIME.0..=PIPE_TIME.1);
        println!("Pipe spawning in {bingus} seconds!");
        spawn_time
            .0
            .set_duration(Duration::from_secs(bingus as u64));
    }
}

fn move_pipes(mut query: Query<&mut LinearVelocity, With<Moving>>, time: Res<Time>) {
    const PIPE_SPEED: f32 = -700.0;

    let delta_secs = time.delta_secs();

    for mut linear_velocity in &mut query {
        linear_velocity.x = PIPE_SPEED * delta_secs;
    }
}

fn despawn_pipes(
    mut commands:Commands,
    query: Query<(Entity, &Transform), With<Moving>>,
){
    const OFFSCREEN_X_DELETE: f32 = -200.0;

    for (entity, transform) in query.iter() {
        if transform.translation.x < OFFSCREEN_X_DELETE {
            commands.entity(entity).despawn();
            //println!("Pipe deleted.")
        }
    }
}

fn award_point(event: On<CollisionEnd>, bird_query: Query<&Controllable>) {
    let sensor = event.collider1;
    let bird = event.collider2;

    if bird_query.contains(bird) {
        println!("{bird} passed through {sensor}, +1 Points")
    }
}
