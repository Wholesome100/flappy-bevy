use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::{PURPLE_400, RED_800},
    prelude::*,
};

/// Plugin for the obstacles the player will interact with, including ground/sky
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_borders, spawn_pipes));
        app.add_systems(FixedUpdate, move_pipes);
    }
}

#[derive(Component)]
pub struct Deadly;

#[derive(Component)]
pub struct Moving;

fn spawn_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let border_shape = Rectangle::new(150.0, 10.0);
    let border_color = ColorMaterial::from_color(RED_800);

    // Entity block for the ground
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color.clone())),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., -32.0, 0.),
        Deadly,
    ));

    // Entity block for the sky
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color)),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., 41.0, 0.),
    ));
}

fn spawn_pipes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // NOTE: Maximum pipe height can be 63.0
    let pipe_shape = Rectangle::new(10., 63.);
    let pipe_color = ColorMaterial::from_color(PURPLE_400);

    commands.spawn((
        Mesh2d(meshes.add(pipe_shape)),
        MeshMaterial2d(materials.add(pipe_color)),
        RigidBody::Kinematic,
        Collider::from(pipe_shape),
        Transform::from_xyz(0., 4.5, 0.),
        Moving,
        Deadly,
    ));
}

fn move_pipes(mut query: Query<&mut LinearVelocity, With<Moving>>, time: Res<Time>) {
    let delta_secs = time.delta_secs();
    for mut linear_velocity in &mut query {
        linear_velocity.x = -700.0 * delta_secs;
    }
}
