use bevy::prelude::*;
use core::default::Default;



#[derive(Resource)]
pub struct BoardConfig {
    pub offset_x: f32,
    pub offset_z: f32,
    pub board_dim: usize,
    pub world_dim: f32,
    pub border_size: f32,
    pub board_height: f32,
    pub piece_height: f32,
    pub piece_hover_height: f32,
    pub piece_scale: f32
}

impl Default for BoardConfig {
    fn default() -> Self{
        BoardConfig {
            offset_x: -5.,
            offset_z: -5.,
            board_dim: 8,
            world_dim: 10.,
            border_size: 0.25,
            board_height: 0.2,
            piece_height: 0.15,
            piece_hover_height: 0.5,
            piece_scale: 0.7
        }
    }
}