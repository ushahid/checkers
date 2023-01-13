use bevy::prelude::*;
use std::f32::consts::PI;
use bevy_mod_picking::{PickingCameraBundle};
use render_3d::CheckersRenderer3dPlugin;
use input_3d::CheckersInput3dPlugin;
use config::*;
use state::{GameState, CheckersState};
use logic::CheckersGameLogicPlugin;
use checkers_events::CheckersEventsPlugin;

mod render_3d;
mod input_3d;
mod config;
mod state;
mod logic;
mod checkers_events;


fn main() {
    let board_config = BoardConfig{..default()};
    let checkers_state = CheckersState::new(board_config.board_dim);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor { width: 1920., height: 1080., title: String::from("Checkers"), mode: WindowMode::Fullscreen, ..default()},
            ..default()
        }))
        .add_state(GameState::Input)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(board_config)
        .insert_resource(checkers_state)
        .add_startup_system(setup)
        .add_plugin(CheckersGameLogicPlugin)
        .add_plugin(CheckersRenderer3dPlugin)
        .add_plugin(CheckersInput3dPlugin)
        .add_plugin(CheckersEventsPlugin)
        .run();
}


// Set up camera and lighting
fn setup(mut commands: Commands) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 10.0, -5.0),
        ..default()
    });

    let distance = 12.5;
    let angle = PI / 4.0;
    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, distance * angle.sin(), distance * angle.cos()).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}