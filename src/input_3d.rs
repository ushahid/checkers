use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, PickingPlugin, InteractablePickingPlugin, HoverEvent, PickableMesh, PickableBundle, SelectionEvent, Hover};
use crate::{
    rendering_3d::{BoardSquareComponent, PieceComponent}, state::{GameState, CheckersState},
    checkers_events::*,
    logic::{InputMove, PossibleMoves, Move}
};


pub struct CheckersInput3dPlugin;


impl Plugin for CheckersInput3dPlugin {
    fn build(&self, app: &mut App){
        app.add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Input).with_system(mark_pickable_pieces))
        .add_system_set(SystemSet::on_exit(GameState::Input).with_system(unmark_pickable_pieces))
        .add_system_set(SystemSet::on_enter(GameState::RestrictedInput).with_system(mark_pickable_pieces))
        .add_system_set(SystemSet::on_exit(GameState::RestrictedInput).with_system(unmark_pickable_pieces))
        .add_system(handle_picking_events.after(mark_pickable_pieces))
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
        mut input_move: ResMut<InputMove>,
        mut game_state: ResMut<State<GameState>>
    ){

    
    let mut board_sq_selected = false;
    let mut piece_deselected = false;

    
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
                            info!("Piece selected: {:?}", piece_comp.pos);
                            input_move.from = Some(piece_comp.pos);
                            input_move.to = None;
                            select_writer.send(PieceSelectEvent{pos: piece_comp.pos});

                        } else if let Ok(board_sq_comp) =  bsc_query.get(*entity){
                            info!("Square selected: {:?}", board_sq_comp.pos);
                            board_sq_selected = true;
                            input_move.to = Some(board_sq_comp.pos);
                            if input_move.from.is_some(){
                                let from = input_move.from.unwrap();
                                let to = input_move.to.unwrap();
                                input_move.from = None;
                                input_move.to = None;
                                game_state.set(GameState::TryMove).unwrap();
                                trymove_writer.send(TryMoveEvent{
                                    from,
                                    to
                                });
                            }
                        } else {
                            panic!("Selected an entity that should not exist");
                        }
                    },
                    SelectionEvent::JustDeselected(entity) => {
                        if let Ok(piece_comp) = pc_query.get(*entity){
                            deselect_writer.send(PieceDeselectEvent{pos: piece_comp.pos});
                            piece_deselected = true;
                        } else if bsc_query.get(*entity).is_ok(){
                            ()
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

    if piece_deselected && !board_sq_selected {
        input_move.from = None;
    }
}


// Function to mark board squares and pieces pickable
fn mark_pickable_pieces(
    mut commands: Commands,
    query: Query<(Entity, &PieceComponent)>,
    checkers_state: Res<CheckersState>,
    sq_query: Query<(Entity, &BoardSquareComponent)>,
    game_state: Res<State<GameState>>,
    possible_moves: Option<Res<PossibleMoves>>,
    mut input_move: ResMut<InputMove>,
    mut select_writer: EventWriter<PieceSelectEvent>,
){
    if *game_state.current() == GameState::RestrictedInput{
        if let Some(m) = possible_moves {
            let source_pos = m.moves[0].from;

            // Mark only the relevant piece pickable
            for (entity, piece) in query.iter(){
                if piece.pos == source_pos{
                    commands.entity(entity).insert(PickableBundle::default());
                    input_move.from = Some(source_pos);
                    select_writer.send(PieceSelectEvent { pos: source_pos });
                    info!("Marked piece {:?} as pickable", source_pos);
                }
            }
            // Mark only the relevant squares pickable
            for (entity, square) in sq_query.iter() {
                if m.moves.contains(&Move{ from: source_pos, to: square.pos}){
                    commands.entity(entity).insert(PickableBundle::default());
                    info!("Marked square {:?} as pickable", square.pos);
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
            match checkers_state.board[square.pos.row][square.pos.col]{
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