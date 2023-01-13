use bevy::prelude::*;
use crate::{
    state::{GameState, CheckersState, PieceType, PieceColor},
    checkers_events::*
};


pub struct CheckersGameLogicPlugin;

impl Plugin for CheckersGameLogicPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::TryMove).with_system(validate_move))
        .add_system_set(SystemSet::on_update(GameState::Move).with_system(handle_move));
    }
}


// check for any possible kills
fn check_kills(_checkers_state: &CheckersState) -> Option<(usize, usize)>{

    return None;
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
        let ans = check_kills(checkers_state);
        if let Some((killer_row, killer_col)) = ans {
            if killer_row != from_row && killer_col != from_col {
                info!{"Invalid move! Kill is mandatory"}
                return false;
            }
        }
    }
    info!("Valid move from ({}, {}) to ({}, {})", from_row, from_col, to_row, to_col);
    return true;
}


fn validate_move(
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
            game_state.set(GameState::Input).unwrap()
        } else {
            move_writer.send(PieceMoveEvent{
                from_row: ev.from_row,
                from_col: ev.from_col,
                to_row: ev.to_row,
                to_col: ev.to_col,
                piece_id: ev.piece_id,
                sq_id: ev.sq_id
            });
            game_state.set(GameState::Move).unwrap();
        }
    }
}


fn handle_move(mut move_event: EventReader<PieceMoveEvent>, mut checkers_state: ResMut<CheckersState>, mut game_state: ResMut<State<GameState>>, mut kill_writer: EventWriter<KillPiece>, mut upgrade_writer: EventWriter<UpgradePiece>){
    for ev in move_event.iter(){
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
                    kill_writer.send(KillPiece { row: mid_row, col: mid_col });
                    info!("Piece killed at: ({}, {})", mid_row, mid_col);
                } else {
                    panic!("Killing own piece");
                }
            }
        }

        // handle upgrades, piece has already been moved
        let mut moving_piece =  checkers_state.board[ev.to_row][ev.to_col].unwrap();
        if (ev.to_row == 0 && moving_piece.typ == PieceType::Man && moving_piece.col == PieceColor::Black) || (ev.to_row == checkers_state.board.len() - 1 && moving_piece.typ == PieceType::Man && moving_piece.col == PieceColor::Red){
            moving_piece.typ = PieceType::King;
            upgrade_writer.send(UpgradePiece { piece_id: ev.piece_id });
            checkers_state.board[ev.to_row][ev.to_col] = Some(moving_piece);
            info!("Made king at: ({}, {})", ev.to_row, ev.to_col);
        }

        checkers_state.turn = match checkers_state.turn {
            PieceColor::Red => PieceColor::Black,
            PieceColor::Black => PieceColor::Red
        };
        game_state.set(GameState::Input).unwrap();
    }
}


