use bevy::prelude::*;
use crate::{logic::{Move, Position}, state::{GameState, CheckersState, PieceType}, checkers_events::TryMoveEvent, alphabeta::{minimax_alpha_beta, TwoPlayerGameState}};
use std::collections::VecDeque;


pub struct CheckersAIPlugin;


impl Plugin for CheckersAIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AIMoves{moves: VecDeque::<Move>::new()})
        .insert_resource(AIStatus{enabled: true})
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


struct JumpNode {
    game_move: Option<Move>,
    children: Option<Vec<JumpNode>>
}

impl JumpNode {
    fn new(game_move: Move) -> Self {
        JumpNode { game_move: Some(game_move), children: None }
    }

    fn build_jump_tree(checkers_state: &CheckersState) -> Self {
        let mut root = JumpNode{game_move: None, children: None};
        let possible_captures = checkers_state.possible_captures();
        // info!("Initial captures: {:?}", possible_captures);
        root.children = Self::build_tree_helper(checkers_state, &possible_captures);
        return root;
    }

    fn build_tree_helper(checkers_state: &CheckersState, next_capture_moves: &Vec<Move>) -> Option<Vec<JumpNode>> {
        if next_capture_moves.len() > 0 {
            let mut children = Vec::new();
            for m in next_capture_moves.iter() {
                let mut node = JumpNode::new(*m);
                let mut next_state = checkers_state.clone();
                let (_, _, next_jumps) = next_state.update_with_move(m);
                node.children = Self::build_tree_helper(&next_state, &next_jumps);
                children.push(node);
            }
            return Some(children);
        }
        return None
    }
}


impl TwoPlayerGameState for CheckersState {
    type GameState = CheckersState;
    type GameMove = Vec<Move>;
    

    fn get_possible_moves(&self) -> Vec<Self::GameMove>{
        let mut move_vectors = Vec::<Self::GameMove>::new();

        // create a tree of jump nodes
        let tree = JumpNode::build_jump_tree(self);
        let mut current = VecDeque::<&JumpNode>::new();
        let mut path = VecDeque::<Move>::new();
        current.push_front(&tree);
        while current.len() > 0 {
            while let Some(node) = current.pop_front(){
                if let Some(m) = node.game_move {
                    path.push_back(m)
                }

                if let Some(ref children) = node.children {
                    for m in children.iter(){
                        current.push_front(m);
                    }
                } else {
                    if path.len() > 0 {
                        let mut v = Vec::<Move>::with_capacity(path.len());
                        for m in path.iter() {
                            v.push(*m);
                        }
                        // info!("Move path is {:?}", v);
                        move_vectors.push(v);
                    }
                    path.pop_back();
                }
            }
        }

        // if no captures are possible
        if move_vectors.len() == 0 {
            for row in 0..self.board.len(){
                for col in 0..self.board.len(){
                    if let Some(piece) = self.board[row][col] {
                        if piece.col == self.turn {
                            let valid_step_moves = self.valid_steps(&Position::new(row, col));
                            for step_move in valid_step_moves {
                                let mut v = Vec::<Move>::new();
                                v.push(step_move);
                                move_vectors.push(v);
                            }
                        }
                    }
                }
            }
        }
        return move_vectors;
    }


    fn next_state_with_move(&self, moves: &Self::GameMove) -> Self::GameState {
        let mut next_state = self.clone();
        for m in moves {
            next_state.update_with_move(&m);
        }
        return next_state;
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