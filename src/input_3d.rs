use bevy::prelude::*;
use bevy_mod_picking::{PickingEvent, PickingPlugin, InteractablePickingPlugin, HoverEvent, PickableMesh, PickableBundle, SelectionEvent};
use crate::{
    render_3d::{BoardSquareComponent, PieceComponent}, state::{GameState, CheckersState},
    checkers_events::*,
};


pub struct CheckersInput3dPlugin;


impl Plugin for CheckersInput3dPlugin {
    fn build(&self, app: &mut App){
        app.add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Input).with_system(mark_pickable_pieces))
        .add_system_set(SystemSet::on_exit(GameState::Input).with_system(unmark_pickable_pieces))
        .add_system_set(SystemSet::on_update(GameState::Input).with_system(handle_picking_events))
        .add_system_set(SystemSet::on_exit(GameState::Move).with_system(handle_move))
        .add_system(bevy::window::close_on_esc);
    }
}


fn dim_material(material_handle: &Handle<StandardMaterial>, materials: &mut ResMut<Assets<StandardMaterial>>, lightness_factor: f32){
    let base_col = &mut (*materials).get_mut(material_handle).unwrap().base_color;
    let color_data = base_col.as_hsla_f32();
    *base_col = Color::Hsla { hue: color_data[0], saturation: color_data[1], lightness: color_data[2] * lightness_factor, alpha: color_data[3]}.as_rgba();
}


fn handle_move(mut move_event: EventReader<PieceMoveEvent>, query: Query<&Handle<StandardMaterial>, With<BoardSquareComponent>>, mut materials: ResMut<Assets<StandardMaterial>>){
    const HOVER_LIGHTNESS_FACTOR: f32 = 0.6;
    for event in move_event.iter(){
        if let Ok(material_handle) = query.get(event.sq_id) {
            dim_material(material_handle, &mut materials, 1.0 / HOVER_LIGHTNESS_FACTOR);
        } else {
            panic!("Invalid entity handle");
        }
    }
}


fn handle_picking_events(
    query: Query<&Handle<StandardMaterial>, With<PickableMesh>>,
    mut events: EventReader<PickingEvent>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    comp_query: Query<(Option<&PieceComponent>, Option<&BoardSquareComponent>)>,
    mut select_writer: EventWriter<PieceSelectEvent>,
    mut deselect_writer: EventWriter<PieceDeselectEvent>,
    mut trymove_writer: EventWriter<TryMoveEvent>,
    mut game_state: ResMut<State<GameState>>
){
    const HOVER_LIGHTNESS_FACTOR: f32 = 0.6;
    let mut piece_selected: bool = false;
    let mut sq_selected: bool = false;
    let mut piece_deselected: bool = false;
    let mut selected_entity:Option<Entity> = None;
    let mut deselected_entity: Option<Entity> = None;
    
    for event in events.iter() {
        match event {
            PickingEvent::Hover(hover_event) => {
                match hover_event {
                    HoverEvent::JustEntered(entity) => {
                        if let Ok(material_handle) = query.get(*entity) {
                            dim_material(material_handle, &mut materials, HOVER_LIGHTNESS_FACTOR);
                        } else {
                            panic!("Invalid entity handle");
                        }
                    },
                    HoverEvent::JustLeft(entity) => {
                        if let Ok(material_handle) = query.get(*entity) {
                            dim_material(material_handle, &mut materials, 1.0 / HOVER_LIGHTNESS_FACTOR);
                        } else {
                            panic!("Invalid entity handle");
                        }
                    }
                }
            },
            PickingEvent::Selection(selection_event) => {
                match selection_event {
                    SelectionEvent::JustSelected(entity) => {
                        if let Ok((piece_comp, _)) = comp_query.get(*entity){
                            if piece_comp.is_some() {
                                piece_selected = true;
                                selected_entity = Some(*entity);
                            } else {
                                sq_selected = true;
                                selected_entity = Some(*entity);
                            }
                        }
                    },
                    SelectionEvent::JustDeselected(entity) => {
                        if let Ok((pc_comp, sq_comp)) = comp_query.get(*entity){
                            if pc_comp.is_some() {
                                piece_deselected = true;
                                deselected_entity = Some(*entity);
                            } else if sq_comp.is_some(){
                                deselected_entity = Some(*entity);
                            } else {
                                panic!("Deselected something that should not exist");
                            }
                        }
                    }
                }
            }
            PickingEvent::Clicked(_) => {
                // println!("Clicked: {:?}", click_event);
                ();
            },
        }
    }

    if piece_selected {
        select_writer.send(PieceSelectEvent{entity_id: selected_entity.unwrap()});
    }

    if piece_deselected && !sq_selected{
        deselect_writer.send(PieceDeselectEvent{entity_id: deselected_entity.unwrap()});
    } else if piece_deselected &&  sq_selected{
        if let Ok((piece_comp, _)) = comp_query.get(deselected_entity.unwrap()){
            if let Ok((_, board_sq)) = comp_query.get(selected_entity.unwrap()){
                let piece_comp = piece_comp.unwrap();
                let board_sq = board_sq.unwrap();
                trymove_writer.send(TryMoveEvent{
                    from_row: piece_comp.row,
                    from_col: piece_comp.col,
                    to_row: board_sq.row,
                    to_col: board_sq.col,
                    piece_id: deselected_entity.unwrap(),
                    sq_id: selected_entity.unwrap()
                });
                game_state.set(GameState::TryMove).unwrap();
            }
        }
        
    }
}


// Function to mark board squares and pieces pickable
fn mark_pickable_pieces(mut commands: Commands, query: Query<(Entity, &PieceComponent)>, checkers_state: Res<CheckersState>, sq_query: Query<(Entity, &BoardSquareComponent)>){
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


// Function to unmark pickable pieces
fn unmark_pickable_pieces(mut commands: Commands, query: Query<Entity, With<PickableMesh>>){
    for entity in query.iter(){
        commands.entity(entity).remove::<PickableBundle>();
    }
}