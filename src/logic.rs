use bevy::prelude::*;
use crate::{
    state::{GameState, CheckersState, PieceType, PieceColor},
    checkers_events::*
};


pub struct CheckersGameLogicPlugin;


impl Plugin for CheckersGameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(MoveFromRes{m: None})
        .add_state(GameState::Input)
        .add_system(handle_try_move)
        .add_system_set(SystemSet::on_update(GameState::Move).with_system(handle_move))
        .add_system_set(SystemSet::on_exit(GameState::RestrictedInput).with_system(remove_resources));
    }
}



#[derive(Resource, Clone, Copy)]
pub struct MoveFrom {
    pub row: usize,
    pub col: usize
}

#[derive(Resource, Clone, Copy)]
pub struct MoveFromRes {
    pub m: Option<MoveFrom>
}

#[derive(Resource)]
pub struct PossibleMoves {
    pub moves: Vec<Move>
}

#[derive(PartialEq)]
pub struct Move {
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize
}


fn remove_resources(mut commands: Commands){
    commands.remove_resource::<PossibleMoves>()
}



// check if kill is possible given location of piece
fn possible_kill_moves_piece(row: usize, col: usize, checkers_state: &CheckersState) -> Vec<Move>{
    let dim:usize = checkers_state.board.len();
    let piece_type: PieceType = checkers_state.board[row][col].unwrap().typ;
    let row_delta_iter = match (piece_type, checkers_state.turn){
        (PieceType::Man, PieceColor::Red) => {
            (1..2).step_by(2)
        },
        (PieceType::Man, PieceColor::Black) => {
            (-1..0).step_by(2)
        },
        (PieceType::King, _) => {
            (-1..2).step_by(2)
        }
    };

    let mut possible_kills: Vec<Move> = Vec::<Move>::new();
    for row_delta in row_delta_iter{
        for col_delta in (-1..2).step_by(2){
            // info!("Testing for {}, {}: {}, {}", row, col, row_delta, col_delta);
            let target_row: i32 = row as i32 + row_delta;
            let target_col: i32 = col as i32 + col_delta;
            let jump_row: i32 = row as i32 + row_delta * 2;
            let jump_col: i32 = col as i32 + col_delta * 2;
            // info!("Checking for {},{}: target({}, {}), jump({}, {})", row, col, target_row, target_col, jump_row, jump_col);
            let is_valid_dim = |pos: i32| -> bool {return  pos >= 0 && pos < dim as i32 };
            if is_valid_dim(target_row) && is_valid_dim(target_col) && is_valid_dim(jump_row) && is_valid_dim(jump_col){
                if let Some(target_piece) = checkers_state.board[target_row as usize][target_col as usize] {
                    if target_piece.col != checkers_state.turn && checkers_state.board[jump_row as usize][jump_col as usize].is_none(){
                        // info!("Valid kill");
                        possible_kills.push(Move{from_row: row, from_col: col, to_row: jump_row as usize, to_col: jump_col as usize});
                    }
                }
            }
            // info!("Invalid kill");
        }
    }
    return possible_kills;
}


// check for any possible kills
fn check_kill_moves(checkers_state: &CheckersState) -> Vec<Move>{
    let mut possible_kills: Vec<Move> = Vec::<Move>::new();
    let dim: usize = checkers_state.board.len();
    for row in 0..dim{
        for col in 0..dim {
            if let Some(piece) = checkers_state.board[row][col]{
                if piece.col == checkers_state.turn {
                    let mut possible_kills_piece:Vec<Move> = possible_kill_moves_piece(row, col, &checkers_state);
                    if possible_kills_piece.len() > 0{
                        possible_kills.append(&mut possible_kills_piece);
                    }
                }
            }
        }
    }
    return possible_kills;
}


fn is_valid_move(from_row: usize, from_col: usize, to_row: usize, to_col: usize, checkers_state: &CheckersState) -> bool{
    // check if the target is empty
    if checkers_state.board[to_row][to_col].is_some(){
        info!{"Invalid move! Target is not empty"}
        return false;
    }

    // check if the source is valid
    if let Some(moving_piece) = checkers_state.board[from_row][from_col]{
        // check if moving your own piece
        if moving_piece.col != checkers_state.turn {
            info!{"Invalid move! Moving opponent's piece"}
            return false;
        }
    } else {
        return false;
    }

    // check if the move is diagonal
    if from_row.abs_diff(to_row) != from_col.abs_diff(to_col){
        info!{"Invalid move! Non-diagonal move"}
        return false;
    }

    if let Some(moving_piece) = checkers_state.board[from_row][from_col]{
        // check if piece moving more than 1 step
        if from_row.abs_diff(to_row) > 1 {
            // check if kill
            if from_row.abs_diff(to_row) == 2 {
                let row_delta: i32 = ((from_row as i32) - (to_row as i32)).signum();
                let col_delta: i32 = ((from_col as i32) - (to_col as i32)).signum();
                if let Some(piece_in_middle) = checkers_state.board[(from_row as i32 - row_delta) as usize][(from_col as i32 - col_delta) as usize]{
                    if piece_in_middle.col == checkers_state.turn{
                        info!{"Invalid move! Jumping over own piece is not possible"}
                        return false;
                    }
                } else {
                    info!{"Invalid move! Cannot move two spaces unless it is a kill"}
                    return false;
                }
            } else {
                info!{"Invalid move! Moving more than 2 spaces is not possible"}
                return false;
            }
        }

        // If man moving in the wrong direction
        if moving_piece.typ == PieceType::Man && (((from_row as i32 - to_row as i32) > 0 && moving_piece.col != PieceColor::Black) || (((from_row as i32) - (to_row as i32) < 0 && moving_piece.col != PieceColor::Red))){
            info!{"Invalid move! Man moving in wrong direction"}
            return false;
        }

        // check for kill sources
        let possible_kills = check_kill_moves(checkers_state);
        if possible_kills.len() > 0{
            if !possible_kills.contains(&Move{from_row, from_col, to_row, to_col}){
                info!{
                    "Invalid move! Kill is mandatory, can only move {}",
                    possible_kills.iter().map(|m|-> String { format!("({}, {}) to ({}, {})", m.from_row, m.from_col, m.to_row, m.to_col) }).collect::<Vec<String>>().join(", ")
                };
                return false;
            }
        }
    }
    info!("Valid move from ({}, {}) to ({}, {})", from_row, from_col, to_row, to_col);
    return true;
}


fn handle_try_move(
    mut trymove_event: EventReader<TryMoveEvent>,
    checkers_state: Res<CheckersState>,
    mut deselect_writer: EventWriter<PieceDeselectEvent>,
    mut move_writer: EventWriter<PieceMoveEvent>,
    mut game_state: ResMut<State<GameState>>
){
    for ev in trymove_event.iter(){
        let is_valid: bool = is_valid_move(ev.from_row, ev.from_col, ev.to_row, ev.to_col, &checkers_state);
        if !is_valid{
            deselect_writer.send(PieceDeselectEvent { entity_id: ev.piece_id });
        } else {
            game_state.set(GameState::Move).unwrap();
            move_writer.send(PieceMoveEvent{
                from_row: ev.from_row,
                from_col: ev.from_col,
                to_row: ev.to_row,
                to_col: ev.to_col,
                piece_id: ev.piece_id,
                sq_id: ev.sq_id
            });
        }
    }
}


fn handle_move(
    mut commands: Commands,
    mut move_event: EventReader<PieceMoveEvent>,
    mut checkers_state: ResMut<CheckersState>,
    mut game_state: ResMut<State<GameState>>,
    mut kill_writer: EventWriter<KillPieceEvent>,
    mut upgrade_writer: EventWriter<UpgradePieceEvent>
){
    for ev in move_event.iter(){
        let mut is_kill: bool = false;

        // update state
        checkers_state.board[ev.to_row][ev.to_col] = checkers_state.board[ev.from_row][ev.from_col];
        checkers_state.board[ev.from_row][ev.from_col] = None;
        
        //  handle kills
        if ev.from_row.abs_diff(ev.to_row) == 2 {
            let row_delta: i32 = ((ev.from_row as i32) - (ev.to_row as i32)).signum();
            let col_delta: i32 = ((ev.from_col as i32) - (ev.to_col as i32)).signum();
            let mid_row: usize = (ev.from_row as i32 - row_delta) as usize;
            let mid_col: usize = (ev.from_col as i32 - col_delta) as usize;

            if let Some(piece_in_middle) = checkers_state.board[mid_row][mid_col] {
                if piece_in_middle.col != checkers_state.turn {
                    checkers_state.board[mid_row][mid_col] = None;
                    kill_writer.send(KillPieceEvent { row: mid_row, col: mid_col });
                    is_kill = true;
                    info!("{:?} killed at: ({}, {})", piece_in_middle.typ, mid_row, mid_col);
                } else {
                    panic!("Cannot kill your own piece");
                }
            }
        }

        // handle upgrades, piece has already been moved
        let mut moving_piece =  checkers_state.board[ev.to_row][ev.to_col].unwrap();
        if (ev.to_row == 0 && moving_piece.typ == PieceType::Man && moving_piece.col == PieceColor::Black) || (ev.to_row == checkers_state.board.len() - 1 && moving_piece.typ == PieceType::Man && moving_piece.col == PieceColor::Red){
            moving_piece.typ = PieceType::King;
            upgrade_writer.send(UpgradePieceEvent { piece_id: ev.piece_id });
            checkers_state.board[ev.to_row][ev.to_col] = Some(moving_piece);
            info!("Made king at: ({}, {})", ev.to_row, ev.to_col);
        }

        
        let mut double_kill: bool = false;

        // switch turn
        if is_kill{
            let possible_kill_moves: Vec<Move> = possible_kill_moves_piece(ev.to_row, ev.to_col, &checkers_state);
            if possible_kill_moves.len() > 0{
                info!("Input restricted!");
                commands.insert_resource(PossibleMoves{moves: possible_kill_moves});
                game_state.set(GameState::RestrictedInput).unwrap();
                double_kill = true;
            }
        }

        if !double_kill {
            checkers_state.turn = match checkers_state.turn {
                PieceColor::Red => PieceColor::Black,
                PieceColor::Black => PieceColor::Red
            };
            game_state.set(GameState::Input).unwrap();

        }
        info!("{:?}'s turn", checkers_state.turn);
    }
}


