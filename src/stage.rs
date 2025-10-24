use avian2d::prelude::*;
use bevy::{
    color::palettes::tailwind::RED_800,
    prelude::*,
};

/// Plugin for the background and ground/sky
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stage);
    }
}


fn spawn_stage(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,)
{
    let border_shape = Rectangle::new(100.0, 10.0);
    let border_color = ColorMaterial::from_color(RED_800);
    // Quick code to test an obstacle entity
    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color.clone())),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., -20.0, 0.),
    ));

    commands.spawn((
        Mesh2d(meshes.add(border_shape)),
        MeshMaterial2d(materials.add(border_color)),
        RigidBody::Static,
        Collider::from(border_shape),
        Transform::from_xyz(0., 20.0, 0.),
    ));
}
