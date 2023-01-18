use bevy::prelude::*;
use std::f32::consts::PI;
use bevy_mod_picking::{PickingCameraBundle};
use rendering_3d::CheckersRendering3dPlugin;
use input_3d::CheckersInput3dPlugin;
use config::*;
use state::CheckersState;
use logic::CheckersGameLogicPlugin;
use checkers_events::CheckersEventsPlugin;
use ai::CheckersAIPlugin;

mod rendering_3d;
mod input_3d;
mod config;
mod state;
mod logic;
mod checkers_events;
mod ai;
mod search;


fn main() {
    let board_config = BoardConfig{..default()};
    let checkers_state = CheckersState::new(board_config.board_dim);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1920.,
                height: 1080.,
                title: String::from("Checkers"),
                mode: WindowMode::BorderlessFullscreen, ..default()
            },
            ..default()
        }))
        .insert_resource(board_config)
        .insert_resource(checkers_state)
        .add_startup_system(setup)
        .add_plugin(CheckersGameLogicPlugin)
        .add_plugin(CheckersRendering3dPlugin)
        .add_plugin(CheckersInput3dPlugin)
        .add_plugin(CheckersEventsPlugin)
        .add_plugin(CheckersAIPlugin)
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
        transform: Transform::from_xyz(0.0, distance * angle.sin(), distance * angle.cos() + 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PickingCameraBundle::default(),
    ));
}