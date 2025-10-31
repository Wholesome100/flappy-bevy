// To build in debug with dynamic linking, run:
// cargo run --features bevy/dynamic_linking
use avian2d::prelude::*;
use bevy::{camera::ScalingMode, prelude::*};

mod bird;
mod obstacles;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource, Default)]
pub struct Score(pub u32);

// Beyond setup, wider collision functions and UI will happen in main
fn main() {
    App::new()
        .insert_resource(Score::default())
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(0.25),
            obstacles::ObstaclePlugin,
            bird::BirdPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, (setup_camera, setup_score_ui).chain())
        .add_systems(OnEnter(GameState::GameOver), game_over_ui)
        .run();
}

/// Startup method to initialize the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 100.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

#[derive(Component)]
struct ScoreText;

fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Percent(50.0),
            ..default()
        },
        ScoreText,
    ));
}

/// UI code for the game over screen
fn game_over_ui(mut commands: Commands, score: Res<Score>) {
    commands.spawn((
        Text::new(format!("GAME OVER\nFinal Score: {}", score.0)),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(35.0),
            ..default()
        },
    ));
}
