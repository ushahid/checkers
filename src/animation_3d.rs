// use bevy::prelude::*;
// use crate::{checkers_events::*, config::BoardConfig, rendering_3d::{PieceComponent, compute_piece_center}, state::GameState, logic::PostAnimationState};


// pub struct CheckersAnimationPlugin;


// impl Plugin for CheckersAnimationPlugin {
//     fn build(&self, app: &mut App){
//         app
//         .add_system(handle_piece_selection)
//         .add_system_set(SystemSet::on_update(GameState::Animating).with_system(cleanup_players_clips))
//         .add_system_set(SystemSet::on_exit(GameState::Move).with_system(handle_move))
//         .add_system_set(SystemSet::on_exit(GameState::Move).with_system(handle_kill))
//         ;
//     }
// }





// fn cleanup_players_clips (
//     mut commands: Commands,
//     query: Query<(Entity, &AnimationPlayer)>,
//     player_data_query: Query<&PlayerData>,
//     mut animation_assets: ResMut<Assets<AnimationClip>>,
//     mut game_state: ResMut<State<GameState>>,
//     post_animation_state: Res<PostAnimationState>
// ) {
//     let mut running = false;
//     for (entity, player) in query.iter() {
//         let data = player_data_query.get(entity).unwrap();
//         if player.elapsed() > data.duration {
//             commands.entity(entity).remove::<AnimationPlayer>();
//             commands.entity(entity).remove::<PlayerData>();
//             animation_assets.remove(data.clip.clone());
//             if data.despawn {
//                 commands.entity(entity).despawn_recursive();
//             }
//         } else {
//             running = true;
//         }
//     }

//     if !running {
//         game_state.set(post_animation_state.state.clone()).unwrap();
//     }
// }


// fn create_translation_clip(entity: &Entity, duration: f32, translations: &Vec<Vec3>) -> AnimationClip {
//     // Create entity path from ID
//     let mut parts = Vec::<Name>::new();
//     parts.push(Name::new(format!("{:?}", entity)));
//     let entity_path = EntityPath{parts};

//     // Create keyframes and timesteps
//     let mut keyframes_vec = Vec::new();
//     for  i in 0..translations.len(){
//         keyframes_vec.push(translations[i]);
//     }

//     let mut kf_timestamps = Vec::new();
//     for  i in 0..keyframes_vec.len(){
//         kf_timestamps.push((i as f32 / keyframes_vec.len() as f32) * duration);
//     }

//     // Build cilp
//     let keyframes = Keyframes::Translation(keyframes_vec);
//     let mut clip = AnimationClip::default();
//     clip.add_curve_to_path(entity_path, VariableCurve{keyframe_timestamps: kf_timestamps, keyframes});
//     return clip;
// }


// fn handle_piece_selection(
//     mut commands: Commands,
//     mut ev: EventReader<PieceSelectEvent>,
//     mut query: Query<(Entity, &PieceComponent)>,
//     board_config: Res<BoardConfig>,
//     mut animation_assets: ResMut<Assets<AnimationClip>>,
//     // mut animations: ResMut<Animations>
// ){
//     for event in ev.iter(){
//         for (entity, piece_component) in query.iter_mut() {
//             if piece_component.pos == event.pos {
//                 let translation = compute_piece_center(piece_component.pos.row, piece_component.pos.col, &board_config);

//                 let duration: f32 = 0.2;
//                 let clip = create_translation_clip(
//                     &entity,
//                     duration, 
//                     &vec![translation, Vec3{x: translation.x, y: translation.y + board_config.piece_hover_height, z: translation.z}]
//                 );
//                 let handle = animation_assets.add(clip);
//                 let  mut player = AnimationPlayer::default();
//                 player.play(handle.clone());
//                 commands.entity(entity).insert(PlayerData{duration: duration, clip: handle.clone(), despawn: false});
//                 commands.entity(entity).insert(player);
//             }
//         }
//     }
// }



// fn handle_move(
//     mut commands: Commands,
//     mut move_event: EventReader<PieceMoveEvent>,
//     mut query: Query<(Entity, &mut Transform, &mut PieceComponent)>,
//     board_config: Res<BoardConfig>,
//     mut animation_assets: ResMut<Assets<AnimationClip>>
// ){
//     for event in move_event.iter() {
//         for (entity, transform, mut piece_component) in query.iter_mut(){
//             if piece_component.pos == event.game_move.from {
//                 piece_component.pos = event.game_move.to;
//                 let center = compute_piece_center(event.game_move.to.row, event.game_move.to.col, &board_config);
//                 let translation = transform.translation;
//                 let is_jump = event.game_move.is_jump();
//                 let duration: f32 = match is_jump {
//                     true => {1.},
//                     false => {0.3}
//                 };

                
//                 let clip = match is_jump{
//                     true => {
//                         create_translation_clip(
//                             &entity,
//                             duration, 
//                             &vec![translation, Vec3{x: center.x, y: center.y + 1., z: center.z}, center]
//                         )
//                     },
//                     false => {
//                         create_translation_clip(
//                             &entity,
//                             duration, 
//                             &vec![translation, center]
//                         )
//                     },
//                 };


//                 let handle = animation_assets.add(clip);
//                 let  mut player = AnimationPlayer::default();
//                 player.play(handle.clone());
//                 commands.entity(entity).insert(PlayerData{duration: duration, clip: handle.clone(), despawn: false});
//                 commands.entity(entity).insert(player);
//             }
//         }
//     }
// }


// fn handle_kill(
//     mut commands: Commands,
//     mut kill_event: EventReader<KillPieceEvent>,
//     query: Query<(Entity, &Transform, &PieceComponent)>,
//     board_config: Res<BoardConfig>,
//     mut animation_assets: ResMut<Assets<AnimationClip>>
// ){
//     for event in kill_event.iter(){
//         for (entity, transform, piece_component) in query.iter(){
//             if piece_component.pos == event.pos {          
//                 let translation = transform.translation;
//                 let duration: f32 = 0.3;
//                 let clip = create_translation_clip(
//                     &entity,
//                     duration, 
//                     &vec![translation, Vec3{  x: translation.x,
//                                                             y: translation.y - (board_config.piece_height + 0.02),
//                                                             z: translation.z }]
//                 );
//                 let handle = animation_assets.add(clip);
//                 let  mut player = AnimationPlayer::default();
//                 player.play(handle.clone());
//                 commands.entity(entity).insert(PlayerData{duration: duration, clip: handle.clone(), despawn: true});
//                 commands.entity(entity).insert(player);
//             }
//         }
//     }
// }