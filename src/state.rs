use bevy::prelude::*;
use crate::logic::{Move, Position};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Input,
    TryMove,
    Move,
    RestrictedInput,
    AIMove
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Man,
    King
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    Black,
    Red
}

#[derive(Debug, Clone, Copy)]
pub struct CheckersPiece {
    pub col: PieceColor,
    pub typ: PieceType
}

#[derive(Resource)]
pub struct CheckersState {
    pub turn: PieceColor,
    pub board: Vec<Vec<Option<CheckersPiece>>>
}


impl CheckersState {
    pub fn new(dim: usize) -> Self {
        let mut board = Vec::new();
        for row in 0..dim {
            let mut board_row = Vec::<Option<CheckersPiece>>::new();
            for col in 0..dim {
                if (row + (col % 2)) % 2 == 0 {
                    if (row as f32) < dim as f32 / 2.0 - 1. {
                        board_row.push(Some(CheckersPiece {col: PieceColor::Red, typ: PieceType::Man}));
                    } else if (row as f32) > dim as f32 / 2.0 {
                        board_row.push(Some(CheckersPiece {col: PieceColor::Black, typ: PieceType::Man}));
                    } else {
                        board_row.push(None);
                    }
                } else {
                    board_row.push(None);
                }
                
            }
            board.push(board_row);
        }
        return CheckersState { turn: PieceColor::Black, board };
    }

    fn at<'a>(&self, pos: &Position) -> Option<CheckersPiece> {
        if self.board[pos.row][pos.col].is_none(){
            return None;
        }
        return Some(self.board[pos.row][pos.col].unwrap());
    }


    fn is_valid_dim(&self, dim: i32) -> bool {
        return  dim >= 0 && dim < self.board.len() as i32
    }


    fn is_empty(&self, pos: &Position) -> bool {
        return self.at(pos).is_none()
    }


    fn candidate_moves(&self, pos: &Position, distance: u32) -> Vec<Move>{
        let mut moves = Vec::<Move>::new();
        let piece = self.at(pos);

        // check if valid piece and piece is owned by the player whose turn it is
        if let Some(piece) = piece {
            if piece.col != self.turn {
                return moves;
            }

            let row_delta_iter = match piece {
                CheckersPiece{col: _, typ: PieceType::King} => {
                    (-1..2).step_by(2)
                },
                CheckersPiece{col: PieceColor::Red, typ: PieceType::Man} => {
                    (1..2).step_by(2)
                },
                CheckersPiece{col: PieceColor::Black, typ: PieceType::Man} => {
                    (-1..0).step_by(2)
                }
            };

            for row_delta in row_delta_iter{
                for col_delta in (-1..2).step_by(2){
                    let row = pos.row as i32 + distance as i32 * row_delta;
                    let col = pos.col as i32 + distance as i32 * col_delta;
                    if self.is_valid_dim(row) && self.is_valid_dim(col){
                        moves.push(Move{ from: *pos, to: Position{row: row as usize, col: col as usize} });
                    }
                }
            }
            return moves;
        }

        return moves;
    }

    fn candidate_jumps(&self, pos: &Position) -> Vec<Move> {
        return self.candidate_moves(pos, 2);
    }
    
    fn candidate_steps(&self, pos: &Position) -> Vec<Move>{
        return self.candidate_moves(pos, 1);
    }

    pub fn valid_jumps(&self, pos: &Position) -> Vec<Move> {
        return self.candidate_jumps(pos)
                    .into_iter()
                    .filter(|m: &Move| -> bool {
                        if !self.is_empty(&m.to){
                            return false;
                        }
                        let middle_pos: Position = m.middle_pos().unwrap();
                        if let Some(piece) = self.at(&middle_pos){
                            if piece.col != self.turn {
                                return true;
                            }
                        }
                        false
                    })
                    .collect()
    }

    pub fn valid_steps(&self, pos: &Position) -> Vec<Move> {
        return self.candidate_steps(pos)
                        .into_iter()
                        .filter(|m| self.is_empty(&m.to))
                        .collect()
    }

    pub fn update_with_move(&mut self, m: &Move) -> (Option<Position>, bool, Vec<Move>){
        let mut capture_pos: Option<Position> = None;
        let mut is_capture: bool = false;
        let mut is_upgrade: bool = false;
        let mut next_capture_moves = Vec::<Move>::new();

        // update board
        self.board[m.to.row][m.to.col] = self.board[m.from.row][m.from.col];
        self.board[m.from.row][m.from.col] = None;

        // if capture
        if let Some(middle_pos) = m.middle_pos() {
            self.board[middle_pos.row][middle_pos.col] = None;
            capture_pos = Some(middle_pos);
            is_capture = true;
        }

        // if upgraded, piece already moved
        if self.final_row(m.to.row) && self.at(&m.to).unwrap().typ == PieceType::Man{
            self.board[m.to.row][m.to.col].unwrap().typ = PieceType::King;
            is_upgrade = true;
        }

        // switch turn
        if is_capture && !is_upgrade{
            next_capture_moves.append(&mut self.valid_jumps(&m.to));
        }

        if next_capture_moves.len() == 0 {
            self.turn = match self.turn {
                PieceColor::Red => PieceColor::Black,
                PieceColor::Black => PieceColor::Red
            };

        }
        return (capture_pos, is_upgrade, next_capture_moves);
    }

    pub fn possible_captures(&self) -> Vec<Move> {
        let mut possible_captures = Vec::<Move>::new();
        for row in 0..self.board.len(){
            for col in 0..self.board.len(){
                if let Some(piece) = self.board[row][col]{
                    if piece.col == self.turn {
                        possible_captures.append(&mut self.valid_jumps(&Position{row, col}));
                    }
                }
            }
        }
        return possible_captures;
    }


    fn final_row(&self, row: usize) -> bool{
        match self.turn {
            PieceColor::Red => {
                return row == 0;
            }
            PieceColor::Black => {
                return row == self.board.len() - 1;
            }
        }
    }
}