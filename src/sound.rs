use bevy::prelude::*;
use crate::checkers_events::*;


pub struct CheckersSoundPlugin;


impl Plugin for CheckersSoundPlugin {
    fn build(&self, app: &mut App){
        app
        .add_system(handle_selection)
        .add_system(handle_capture)
        .add_system(handle_move);
    }
}

fn handle_selection(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<PieceSelectEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("select.mp3");
        audio.play_with_settings(sound, PlaybackSettings { volume: 0.8, ..default() });
    }
}


fn handle_capture(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<KillPieceEvent>){
    for _ in events.iter(){
        let sound = asset_server.load("capture.mp3");
        audio.play(sound);
    }
}


fn handle_move(audio: Res<Audio>, asset_server: Res<AssetServer>, mut events: EventReader<PieceMoveEvent>){
    for ev in events.iter(){
        if !ev.game_move.is_jump(){
            let sound = asset_server.load("move.mp3");
            audio.play_with_settings(sound, PlaybackSettings { volume: 0.1, ..default() });
        }
    }
}