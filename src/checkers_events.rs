use bevy::prelude::*;
use crate::{logic::{Move, Position}, state::PieceColor};

pub struct CheckersEventsPlugin;

impl Plugin for CheckersEventsPlugin {
    fn build(&self, app: &mut App){
        app
        .add_event::<ButtonSelectEvent>()
        .add_event::<InvalidMoveEvent>()
        .add_event::<VictoryEvent>()
        .add_event::<HighlightEntityEvent>()
        .add_event::<RemoveHighlightEntityEvent>()
        .add_event::<UpgradePieceEvent>()
        .add_event::<KillPieceEvent>()
        .add_event::<PieceMoveEvent>()
        .add_event::<TryMoveEvent>()
        .add_event::<PieceSelectEvent>()
        .add_event::<PieceDeselectEvent>();
    }
}


pub struct HighlightEntityEvent {
    pub entity_id: Entity
}

pub struct RemoveHighlightEntityEvent {
    pub entity_id: Entity
}


pub struct TryMoveEvent{
    pub game_move: Move
}

pub struct PieceMoveEvent{
    pub game_move: Move
}

pub struct KillPieceEvent {
    pub pos: Position
}

pub struct PieceSelectEvent{
    pub pos: Position
}


pub struct PieceDeselectEvent{
    pub pos: Position
}


pub struct UpgradePieceEvent {
    pub pos: Position
}



pub struct VictoryEvent {
    pub winner: PieceColor
}

pub struct ButtonSelectEvent;

pub struct InvalidMoveEvent;

