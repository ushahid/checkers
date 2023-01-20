use bevy::prelude::*;
use crate::{logic::Move, state::{GameState, CheckersState, PieceType}, checkers_events::TryMoveEvent, alphabeta::{minimax_alpha_beta, TwoPlayerGameState}};
use std::collections::VecDeque;


pub struct CheckersAIPlugin;


impl Plugin for CheckersAIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AIMoves{moves: VecDeque::<Move>::new()})
        .insert_resource(AIStatus{enabled: false})
        .add_system_set(SystemSet::on_update(GameState::AIMove).with_system(make_ai_move));
    }
}


#[derive(Resource)]
pub struct AIStatus {
    pub enabled: bool
}


#[derive(Resource)]
struct AIMoves {
    moves: VecDeque<Move>
}


fn make_ai_move(mut ai_moves: ResMut<AIMoves>, mut trymove_writer: EventWriter<TryMoveEvent>, mut game_state: ResMut<State<GameState>>, checkers_state: Res<CheckersState>){
    if let Some(m) = ai_moves.moves.pop_front() {
        game_state.set(GameState::TryMove).unwrap();
        trymove_writer.send(TryMoveEvent{
            game_move: m
        });
    } else {
        for m in find_best_moves(&checkers_state){
            ai_moves.moves.push_back(m);
        }
    }
}


fn find_best_moves(state: &CheckersState) -> Vec<Move>{
    let (_, best_move) = minimax_alpha_beta(state, 10, f32::NEG_INFINITY, f32::INFINITY, true);
    return best_move.unwrap();
}


impl TwoPlayerGameState for CheckersState {
    type GameState = CheckersState;
    type GameMove = Vec<Move>;
    

    fn get_possible_moves(&self) -> Vec<Self::GameMove>{
        let moves: Vec<Vec<Move>> = Vec::new();
        for row in 0..self.board.len(){
            for col in 0..self.board.len(){
            }
        }

        return moves;
    }


    fn next_state_with_move(&self, m: &Self::GameMove) -> &Self::GameState {
        return self;
    }


    fn score_state(&self) -> f32 {
        let mut score = 0.;
        let mut my_pieces = 0;
        let mut opponent_pieces = 0;

        for row in 0..self.board.len(){
            for col in 0..self.board.len(){
                if let Some(piece) =  self.board[row][col]{
                    let delta = match piece.typ {
                        PieceType::King => {
                            10.
                        },
                        PieceType::Man => {
                            5.
                        }
                    };
                    if piece.col == self.turn {
                        score += delta;
                        my_pieces += 1;
                    } else {
                        score -= delta;
                        opponent_pieces += 1;
                    }
                }
            }
        }

        if my_pieces == 0 {
            return f32::NEG_INFINITY;
        }

        if opponent_pieces == 0 {
            return f32::INFINITY;
        }

        return score;
    }
}