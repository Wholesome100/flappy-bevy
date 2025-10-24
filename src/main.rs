// To build in debug with dynamic linking, run:
// cargo run --features bevy/dynamic_linking
use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

mod bird;

// Beyond setup, wider collision functions and UI will happen in main
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(0.25),
            bird::BirdPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

/// Startup method to initialize the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.10,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
