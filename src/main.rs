// To build in debug with dynamic linking, run:
// cargo run --features bevy/dynamic_linking
use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

mod bird;
mod obstacles;

// Beyond setup, wider collision functions and UI will happen in main
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(0.25),
            obstacles::StagePlugin,
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
            scaling_mode: ScalingMode::Fixed {
                width: 1080.,
                height: 720.,
            },
            scale: 0.20,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
