use bevy::prelude::*;
use crate::logic::Move;

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
}