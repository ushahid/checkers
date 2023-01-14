use bevy::prelude::*;


pub struct CheckersEventsPlugin;

impl Plugin for CheckersEventsPlugin {
    fn build(&self, app: &mut App){
        app
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
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
    pub piece_id: Entity,
    pub sq_id: Entity
}

pub struct PieceMoveEvent{
    pub from_row: usize,
    pub from_col: usize,
    pub to_row: usize,
    pub to_col: usize,
    pub piece_id: Entity,
    pub sq_id: Entity
}

pub struct KillPieceEvent {
    pub row: usize,
    pub col: usize
}

pub struct PieceSelectEvent{
    pub entity_id: Entity
}


pub struct PieceDeselectEvent{
    pub entity_id: Entity
}


pub struct UpgradePieceEvent {
    pub piece_id: Entity
}
