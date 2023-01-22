use bevy::prelude::*;
use crate::{
    state::{GameState, CheckersState, PieceColor},
    checkers_events::*,
    ai::AIStatus
};


pub struct CheckersGameLogicPlugin;


impl Plugin for CheckersGameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PostAnimationState{state: GameState::Menu})
        .insert_resource(InputMove{from: None, to: None})
        .insert_resource(PossibleMoves{moves: None})
        .add_state(GameState::Menu)
        .add_system_set(SystemSet::on_update(GameState::TryMove).with_system(handle_try_move))
        .add_system_set(SystemSet::on_update(GameState::Move).with_system(handle_move))
        .add_system(handle_game_over);
    }
}


#[derive(Resource)]
pub struct PostAnimationState {
    pub state: GameState
}


#[derive(Resource)]
pub struct InputMove {
    pub from: Option<Position>,
    pub to: Option<Position>
}

#[derive(Resource)]
pub struct PossibleMoves {
    pub moves: Option<Vec<Move>>
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position
}

impl Move {
    pub fn is_jump(&self) -> bool {
        if (self.from.row as i32 - self.to.row as i32).abs() != 2 {
            return false;
        }
        if (self.from.col as i32 - self.to.col as i32).abs() != 2 {
            return false;
        }
        true
    }

    pub fn middle_pos(&self) -> Option<Position> {
        if !self.is_jump() {
            return None;
        }

        let row_delta: i32 = ((self.from.row as i32) - (self.to.row as i32)).signum();
        let col_delta: i32 = ((self.from.col as i32) - (self.to.col as i32)).signum();
        
        return Some(Position::new((self.from.row as i32 - row_delta) as usize, (self.from.col as i32 - col_delta) as usize));
    }
}


fn handle_game_over(mut win_reader: EventReader<VictoryEvent>){

    for ev in win_reader.iter() {
        match ev.winner {
            PieceColor::Black => info!{"Black Won!"},
            PieceColor::Red => info!{"Red Won!"}
        }
    }
}


fn is_valid_move(m: &Move, checkers_state: &CheckersState, move_from: Option<Position>) -> bool {
    if let Some(move_from) = move_from {
        if move_from != m.from{
            info! {"Invalid move! Can only move {:?}", move_from}
            return false;
        }
    }
    let possible_jumps = checkers_state.possible_captures();
    if possible_jumps.len() > 0 {
        if possible_jumps.contains(m){
            return true;
        } else {
            info!{
                "Invalid move! Capture is mandatory, can only move {}",
                possible_jumps.iter().map(|m|-> String { format!("{:?} to {:?}", m.from, m.to) }).collect::<Vec<String>>().join(", ")
            };
            return false;
        }
    } else if checkers_state.valid_steps(&m.from, checkers_state.turn).contains(m){
        return true;
    }
    return false;
}


fn handle_try_move(
    mut trymove_event: EventReader<TryMoveEvent>,
    checkers_state: Res<CheckersState>,
    mut move_writer: EventWriter<PieceMoveEvent>,
    mut invalid_writer: EventWriter<InvalidMoveEvent>,
    mut game_state: ResMut<State<GameState>>,
    possible_moves: Res<PossibleMoves>
){
    for ev in trymove_event.iter(){
        let move_from = match possible_moves.moves {
            Some(ref m) => Some(m[0].from),
            None => None
        };
        
        let is_valid: bool = is_valid_move(&ev.game_move, &checkers_state, move_from);
        if !is_valid{
            invalid_writer.send(InvalidMoveEvent);
            info!("Invalid move {:?}", ev.game_move);
            if possible_moves.moves.is_some(){
                game_state.set(GameState::RestrictedInput).unwrap();
            } else {
                game_state.set(GameState::Input).unwrap();
            }

        } else {
            info!("Valid move {:?}", ev.game_move);
            move_writer.send(PieceMoveEvent{
                game_move: ev.game_move
            });
            game_state.set(GameState::Move).unwrap();
        }
    }
}


pub fn handle_move(
    mut move_event: EventReader<PieceMoveEvent>,
    mut checkers_state: ResMut<CheckersState>,
    mut game_state: ResMut<State<GameState>>,
    mut kill_writer: EventWriter<KillPieceEvent>,
    mut upgrade_writer: EventWriter<UpgradePieceEvent>,
    ai_status: Res<AIStatus>,
    mut possible_moves: ResMut<PossibleMoves>,
    mut win_writer: EventWriter<VictoryEvent>,
    mut post_animation_state: ResMut<PostAnimationState>
){
    for ev in move_event.iter(){
        let (capture_pos, is_upgrade, next_capture_moves) = checkers_state.update_with_move(&ev.game_move);
        if let Some(mid_pos) = capture_pos {
            kill_writer.send(KillPieceEvent { pos: mid_pos });
        }

        if is_upgrade {
            upgrade_writer.send(UpgradePieceEvent { pos: ev.game_move.to});
            info!("Upgraded!");
        }


        let mut next_input_state: GameState = GameState::Input;
        if capture_pos.is_some() &&  !is_upgrade && next_capture_moves.len() > 0 {
            possible_moves.moves = Some(next_capture_moves);
            next_input_state = GameState::RestrictedInput;
        } else {
            possible_moves.moves = None;
        }

        if checkers_state.is_loser(checkers_state.turn){
            let winner = match checkers_state.turn {
                PieceColor::Black => PieceColor::Red,
                PieceColor::Red => PieceColor::Black
            };
            win_writer.send(VictoryEvent { winner });
            post_animation_state.state = GameState::GameOver;
        } else {
            if ai_status.enabled{
                match checkers_state.turn {
                    PieceColor::Red => {post_animation_state.state = GameState::AIMove},
                    PieceColor::Black => {post_animation_state.state = next_input_state}
                }
            } else {
                post_animation_state.state = next_input_state
            }
        }
        game_state.set(GameState::Animating).unwrap();
    }
}