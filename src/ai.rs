use bevy::{prelude::*, tasks::AsyncComputeTaskPool, tasks::Task};
use crate::{logic::{Move, Position}, state::{GameState, CheckersState, PieceType, PieceColor}, checkers_events::TryMoveEvent, alphabeta::{minimax_alpha_beta, TwoPlayerGameState}};
use std::{collections::VecDeque};
use futures_lite::future;


pub struct CheckersAIPlugin;



impl Plugin for CheckersAIPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AIMoves{moves: VecDeque::<Move>::new()})
        .insert_resource(AIStatus{enabled: true})
        .add_system_set(SystemSet::on_enter(GameState::AIMove).with_system(queue_compute_move))
        .add_system_set(SystemSet::on_update(GameState::AIMove).with_system(make_ai_move.after(add_ai_move)))
        .add_system_set(SystemSet::on_update(GameState::AIMove).with_system(add_ai_move));
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

#[derive(Component)]
struct ComputeMove {
    task: Task<Vec<Move>>
}


fn queue_compute_move(mut commands: Commands, ai_moves: Res<AIMoves>, checkers_state: Res<CheckersState>){
    if ai_moves.moves.len() == 0 {
        info!("Queueing up compute move");
        let pool = AsyncComputeTaskPool::get();
        let state_clone = checkers_state.clone();
        let task: Task<Vec<Move>> = pool.spawn(async move {
            find_best_moves(&state_clone)
        });
        commands.spawn(ComputeMove{task});
    }
}


fn add_ai_move(
    mut commands: Commands,
    mut compute_tasks: Query<(Entity, &mut ComputeMove)>,
    mut ai_moves: ResMut<AIMoves>
){
    for (entity, mut compute_task) in &mut compute_tasks{
        if let Some(best_moves) = future::block_on(future::poll_once(&mut compute_task.task)){
            info!("Move computed");
            for m in best_moves {
                ai_moves.moves.push_back(m);
            }
            commands.entity(entity).remove::<ComputeMove>();
        }
    }
}


fn make_ai_move(
    mut ai_moves: ResMut<AIMoves>,
    mut trymove_writer: EventWriter<TryMoveEvent>,
    mut game_state: ResMut<State<GameState>>,
){
    if let Some(m) = ai_moves.moves.pop_front() {
        info!("Adding move");
        game_state.set(GameState::TryMove).unwrap();
        trymove_writer.send(TryMoveEvent{
            game_move: m
        });
    }
}


fn find_best_moves(state: &CheckersState) -> Vec<Move>{
    let (_, best_move) = minimax_alpha_beta(state, 10, f32::NEG_INFINITY, f32::INFINITY, true, &state.turn);
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
    type Player = PieceColor;
    

    fn get_possible_moves(&self) -> Vec<Self::GameMove>{
        let mut move_vectors = Vec::<Self::GameMove>::new();

        // create a tree of jump nodes
        let tree = JumpNode::build_jump_tree(self);
        let mut current = VecDeque::<(&JumpNode, Option<Vec<Move>>)>::new();
        current.push_front((&tree, None));
        while current.len() > 0 {
            while let Some(val) = current.pop_front(){
                let (node, path) = val;

                if let Some(ref children) = node.children {
                    for child_node in children.iter(){
                        let mut new_path = match path {
                            Some(ref p) => p.clone(),
                            None => Vec::new()
                        };
                        new_path.push(child_node.game_move.unwrap());
                        current.push_front((child_node, Some(new_path)));
                    }
                } else {
                    if let Some(p) = path {
                        let mut v = Vec::<Move>::with_capacity(p.len());
                        for m in p.iter() {
                            v.push(*m);
                        }
                        move_vectors.push(v);
                    }
                }
            }
        }

        // if no captures are possible
        if move_vectors.len() == 0 {
            for row in 0..self.board.len(){
                for col in 0..self.board.len(){
                    if let Some(piece) = self.board[row][col] {
                        if piece.col == self.turn {
                            let valid_step_moves = self.valid_steps(&Position::new(row, col), self.turn);
                            for step_move in valid_step_moves {
                                let mut v = Vec::<Move>::with_capacity(1);
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
        for m in moves.iter() {
            next_state.update_with_move(m);
        }
        return next_state;
    }


    fn score_state(&self, turn: &Self::Player) -> f32 {
        let mut my_men = 0.;
        let mut my_kings = 0.;
        let mut opp_men = 0.;
        let mut opp_kings = 0.;
        let mut opp_kings_middle = 0.;
        let mut my_kings_middle = 0.;


        for row in 0..self.board.len(){
            for col in 0..self.board.len(){
                if let Some(piece) =  self.board[row][col]{
                    if *turn == piece.col {
                        match piece.typ {
                            PieceType::Man => {
                                my_men += 1.;
                            }
                            PieceType::King => {
                                my_kings += 1.;
                                if self.is_in_middle(Position::new(row, col)){
                                    my_kings_middle += 1.;
                                }
                            }
                        }
                    } else {
                        match piece.typ {
                            PieceType::Man => {
                                opp_men += 1.;
                            }
                            PieceType::King => {
                                opp_kings += 1.;
                                if self.is_in_middle(Position::new(row, col)){
                                    opp_kings_middle += 1.;
                                }
                            }
                        }
                    }
                }
            }
        }

        let my_pieces = my_men + my_kings;
        let opp_pieces = opp_men + opp_kings;

        if my_pieces as i32 == 0 {
            return f32::MIN;
        } else if opp_pieces as i32 == 0 {
            return f32::MAX;
        }
        
        let mut score = 0.;
        score += my_men * 50.;
        score += my_kings * 75.;

        score -= opp_men * 50.;
        score -= opp_kings * 75.;

        score += my_kings_middle * 2.;
        score -= opp_kings_middle * 2.;

        return score;
    }
    
    fn is_game_over(&self) -> bool {
        return self.get_winner().is_some();
    }
}