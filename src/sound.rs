use bevy::prelude::*;
use crate::{checkers_events::*, ai::AIStatus, state::PieceColor};


pub struct CheckersSoundPlugin;


impl Plugin for CheckersSoundPlugin {
    fn build(&self, app: &mut App){
        app
        .add_system(handle_selection)
        .add_system(handle_capture)
        .add_system(handle_move)
        .add_system(handle_victory)
        .add_system(handle_invalid_input)
        .add_system(handle_button_select);
    }
}

fn handle_selection(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<PieceSelectEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("sounds/select.mp3");
        audio.play_with_settings(sound, PlaybackSettings { volume: 0.8, ..default() });
    }
}


fn handle_capture(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<KillPieceEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("sounds/capture.mp3");
        audio.play_with_settings(sound, PlaybackSettings { volume: 0.5, ..default() });
    }
}


fn handle_move(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<PieceMoveEvent>){
    for ev in events.iter(){
        if !ev.game_move.is_jump(){
            let sound = asset_server.load("sounds/move.mp3");
            audio.play_with_settings(sound, PlaybackSettings { volume: 0.1, ..default() });
        }
    }
}

fn handle_button_select(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<ButtonSelectEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("sounds/click.mp3");
        audio.play_with_settings(sound, PlaybackSettings { volume: 1.0, ..default() });
    }
}

fn handle_invalid_input(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<InvalidMoveEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("sounds/error.mp3");
        audio.play_with_settings(sound, PlaybackSettings { volume: 0.1, ..default() });
    }
}


fn handle_victory(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<VictoryEvent>, ai_status: Res<AIStatus>){
    for ev in events.iter(){
        let mut sound = asset_server.load("sounds/celebration.mp3");
        if ai_status.enabled && ev.winner == PieceColor::Red {
            sound = asset_server.load("sounds/loss.mp3");
        }
        audio.play_with_settings(sound, PlaybackSettings { volume: 0.5, ..default() });
    }
}


