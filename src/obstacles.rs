use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::RED_800,
    prelude::*,
};

/// Plugin for the obstacles the player will interact with, including ground/sky
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_borders);
    }
}

#[derive(Component)]
struct Deadly;

fn spawn_borders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,)
{
    let border_shape = Rectangle::new(150.0, 10.0);
    let border_color = ColorMaterial::from_color(RED_800);

    // Entity block for the ground
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color.clone())),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., -32.0, 0.),
    ));

    // Entity block for the sky
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color)),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., 41.0, 0.),
        Deadly
    ));
}
