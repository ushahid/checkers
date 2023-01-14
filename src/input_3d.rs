use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, PickingPlugin, InteractablePickingPlugin, HoverEvent, PickableMesh, PickableBundle, SelectionEvent, Hover};
use crate::{
    rendering_3d::{BoardSquareComponent, PieceComponent}, state::{GameState, CheckersState},
    checkers_events::*,
    logic::{MoveFromRes, MoveFrom, PossibleMoves, Move}
};


pub struct CheckersInput3dPlugin;


impl Plugin for CheckersInput3dPlugin {
    fn build(&self, app: &mut App){
        app.add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Input).with_system(mark_pickable_pieces))
        .add_system_set(SystemSet::on_enter(GameState::RestrictedInput).with_system(mark_pickable_pieces))
        .add_system_set(SystemSet::on_exit(GameState::Input).with_system(unmark_pickable_pieces))
        .add_system_set(SystemSet::on_exit(GameState::RestrictedInput).with_system(unmark_pickable_pieces))
        .add_system_set(SystemSet::on_update(GameState::RestrictedInput).with_system(handle_picking_events))
        .add_system_set(SystemSet::on_update(GameState::Input).with_system(handle_picking_events))
        .add_system(bevy::window::close_on_esc);
    }
}


fn handle_picking_events(
        pc_query: Query<&PieceComponent>,
        bsc_query: Query<&BoardSquareComponent>,
        mut events: EventReader<PickingEvent>,
        mut select_writer: EventWriter<PieceSelectEvent>,
        mut deselect_writer: EventWriter<PieceDeselectEvent>,
        mut highlight_writer: EventWriter<HighlightEntityEvent>,
        mut remove_highlight_writer: EventWriter<RemoveHighlightEntityEvent>,
        mut trymove_writer: EventWriter<TryMoveEvent>,
        mut move_from: ResMut<MoveFromRes>,
    ){

    let mut piece_selected: bool = false;
    let mut sq_selected: bool = false;
    let mut piece_deselected: bool = false;

    let mut selected_entity:Option<Entity> = None;
    let mut deselected_entity:Option<Entity> = None;

    
    for event in events.iter() {
        match event {
            PickingEvent::Hover(hover_event) => {
                match hover_event {
                    HoverEvent::JustEntered(entity) => {
                        highlight_writer.send(HighlightEntityEvent{entity_id: *entity})

                    },
                    HoverEvent::JustLeft(entity) => {
                        remove_highlight_writer.send(RemoveHighlightEntityEvent{entity_id: *entity})
                    }
                }
            },
            PickingEvent::Selection(selection_event) => {
                match selection_event {
                    SelectionEvent::JustSelected(entity) => {
                        if let Ok(piece_comp) = pc_query.get(*entity){
                            piece_selected = true;
                            selected_entity = Some(*entity);
                            move_from.m = Some(MoveFrom{row: piece_comp.row, col: piece_comp.col});

                        } else if bsc_query.get(*entity).is_ok(){
                            sq_selected = true;
                            selected_entity = Some(*entity);
                        } else {
                            panic!("Selected an entity that should not exist");
                        }
                    },
                    SelectionEvent::JustDeselected(entity) => {
                        if pc_query.get(*entity).is_ok(){
                            piece_deselected = true;
                            deselected_entity = Some(*entity);
                        } else if bsc_query.get(*entity).is_ok(){
                            deselected_entity = Some(*entity);
                        } else {
                            panic!("Deselected an entity that should not exist");
                        }
                    }
                }
            }
            PickingEvent::Clicked(_) => {
                ();
            },
        }
    }

    if piece_selected {
        select_writer.send(PieceSelectEvent{entity_id: selected_entity.unwrap()});
    }


    if piece_deselected && !sq_selected{
        deselect_writer.send(PieceDeselectEvent{entity_id: deselected_entity.unwrap()});
    }
    
    if sq_selected &&  move_from.m.is_some(){
        let m = move_from.as_mut().m.unwrap();
        let board_sq = bsc_query.get(selected_entity.unwrap()).unwrap();
        move_from.m = None;
        trymove_writer.send(TryMoveEvent{
            from_row: m.row,
            from_col: m.col,
            to_row: board_sq.row,
            to_col: board_sq.col,
            piece_id: deselected_entity.unwrap(),
            sq_id: selected_entity.unwrap()
        });
    }
}


// Function to mark board squares and pieces pickable
fn mark_pickable_pieces(
    mut commands: Commands,
    query: Query<(Entity, &PieceComponent)>,
    checkers_state: Res<CheckersState>,
    sq_query: Query<(Entity, &BoardSquareComponent)>,
    game_state: Res<State<GameState>>,
    possible_moves: Option<Res<PossibleMoves>>
){
    if *game_state.current() == GameState::RestrictedInput{
        if let Some(m) = possible_moves {
            let source_row: usize = m.moves[0].from_row;
            let source_col: usize = m.moves[0].from_col;

            // Mark only the relevant piece pickable
            for (entity, piece) in query.iter(){
                if piece.row == source_row && piece.col == source_col{
                    commands.entity(entity).insert(PickableBundle::default());
                    info!("Marked piece ({}, {}) as pickable", piece.row, piece.col);
                }
            }
            // Mark only the relevant squares pickable
            for (entity, square) in sq_query.iter() {
                if m.moves.contains(&Move{ from_row: source_row, from_col: source_col, to_row: square.row, to_col: square.col}){
                    commands.entity(entity).insert(PickableBundle::default());
                    info!("Marked square ({}, {}) as pickable", square.row, square.col);
                }
            }
        }
    } else {
        // Mark relevant pieces pickable
        for (entity, piece) in query.iter(){
            if piece.color == checkers_state.turn {
                commands.entity(entity).insert(PickableBundle::default());
            }
        }

        // Mark relevant board squares pickable
        for (entity, square) in sq_query.iter() {
            match checkers_state.board[square.row][square.col]{
                None => {
                    commands.entity(entity).insert(PickableBundle::default());
                }
                Some(_) => ()
            }
        }
    }
}


// Function to unmark pickable pieces
fn unmark_pickable_pieces(
    mut commands: Commands,
    query: Query<Entity, With<PickableMesh>>,
    hover_query: Query<&Hover>,
    mut remove_highlight_event: EventWriter<RemoveHighlightEntityEvent>,
    mut events: ResMut<Events<PickingEvent>>
){
    for entity in query.iter(){
        if let Ok(hover) = hover_query.get(entity){
            if hover.hovered(){
                remove_highlight_event.send(RemoveHighlightEntityEvent { entity_id: entity });
            }
        }
        commands.entity(entity).remove::<PickableBundle>();
    }
    events.clear();
}