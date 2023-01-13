use bevy::prelude::*;
use crate::{
    config::BoardConfig,
    state::*,
    checkers_events::{PieceSelectEvent, PieceDeselectEvent, PieceMoveEvent, KillPiece, UpgradePiece}
};



pub struct CheckersRenderer3dPlugin;


impl Plugin for CheckersRenderer3dPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(setup_board)
        .add_system(handle_piece_selection)
        .add_system(handle_piece_deselection)
        .add_system(handle_move)
        .add_system(handle_kill)
        .add_system(handle_upgrade);
    }
}


#[derive(Component, Debug)]
pub struct PieceComponent{
    pub row: usize,
    pub col: usize,
    pub color: PieceColor,
    pub typ: PieceType
}


#[derive(Component, Debug)]
pub struct BoardComponent;


#[derive(Component, Debug)]
pub struct BoardSquareComponent {
    pub row: usize,
    pub col: usize
}


fn handle_upgrade(
    mut commands: Commands,
    mut upgrade_event: EventReader<UpgradePiece>,
    mut query: Query<&mut PieceComponent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_config: Res<BoardConfig>

){

    for event in upgrade_event.iter(){
        let sq_dim: f32 = (board_config.world_dim - (board_config.border_size * 2.0)) / board_config.board_dim as f32;
        let scaled_sq_dim: f32 = board_config.piece_scale * sq_dim;
        if let Ok((mut piece_component)) = query.get_mut(event.piece_id){
            piece_component.typ = PieceType::King;
            let color = match piece_component.color {
                PieceColor::Black => Color::rgb(0.25, 0.25, 0.25),
                PieceColor::Red => Color::RED,
            };
            let child = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box{
                                                            min_x: - scaled_sq_dim / 2.0,
                                                            max_x: scaled_sq_dim / 2.0,
                                                            min_y: - board_config.piece_height / 2.0,
                                                            max_y: (board_config.piece_height / 2.0) * 1.75,
                                                            min_z: - scaled_sq_dim / 2.0,
                                                            max_z: scaled_sq_dim / 2.0
                                                    })),
                material: materials.add(color.into()),
                transform: Transform::from_scale(Vec3{x: 0.8, y: 1.0, z: 0.8}),
                ..default()
            }).id();
            commands.entity(event.piece_id).push_children(&[child]);

        }
    }
}


fn handle_kill(mut commands: Commands, mut kill_event: EventReader<KillPiece>, query: Query<(Entity, &PieceComponent)>){
    for event in kill_event.iter(){
        for (entity, piece_component) in query.iter(){
            if piece_component.row == event.row && piece_component.col == event.col {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}



fn handle_move(mut query: Query<(&mut Transform, &mut PieceComponent)>, mut move_event: EventReader<PieceMoveEvent>,  board_config: Res<BoardConfig>){
    for event in move_event.iter(){
        if let Ok((mut transform, mut piece_comp)) = query.get_mut(event.piece_id){
            let center = compute_piece_center(event.to_row, event.to_col, &board_config);
            transform.translation.x = center.x;
            transform.translation.y = center.y;
            transform.translation.z = center.z;
            piece_comp.row = event.to_row;
            piece_comp.col = event.to_col;
        }
    }
}



fn compute_piece_center(row: usize, col: usize, board_config: &BoardConfig) -> Vec3{
    let sq_dim: f32 = (board_config.world_dim - (board_config.border_size * 2.0)) / board_config.board_dim as f32;
    let scaled_sq_dim: f32 = board_config.piece_scale * sq_dim;
    let sq_offset_x = board_config.offset_x + board_config.border_size + col as f32 * sq_dim;
    let sq_offset_z = board_config.offset_z + board_config.border_size + row as f32 * sq_dim;
    let sq_center_x = sq_offset_x + (((1. - board_config.piece_scale) / 2.0) * sq_dim) + scaled_sq_dim / 2.0;
    let sq_center_z = sq_offset_z + (((1. - board_config.piece_scale) / 2.0) * sq_dim) + scaled_sq_dim / 2.0;
    return Vec3 {x: sq_center_x, y: (board_config.board_height + board_config.piece_height) / 2.0, z: sq_center_z}
}


fn handle_piece_selection(mut query: Query<&mut Transform, With<PieceComponent>>, board_config: Res<BoardConfig>, mut ev: EventReader<PieceSelectEvent>){
    for event in ev.iter(){
        if let Ok(mut transform) = query.get_mut(event.entity_id){
            transform.translation.y += board_config.piece_hover_height;
        }
    }
}


fn handle_piece_deselection(mut query: Query<(&mut Transform, &mut PieceComponent)>, board_config: Res<BoardConfig>, mut ev: EventReader<PieceDeselectEvent>){
    for event in ev.iter(){
        if let Ok((mut transform, piece)) = query.get_mut(event.entity_id){
            let position = compute_piece_center(piece.row, piece.col, &board_config);
            transform.translation.y = position.y;
        }
    }
}


// System to add board and pieces
fn setup_board(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, board_config: Res<BoardConfig>, checkers_state: Res<CheckersState>){
    add_board(&mut commands, &mut meshes, &mut materials, &board_config);
    add_pieces(&mut commands, &mut meshes, &mut materials, &board_config, &checkers_state);
}


// Function to add pieces using basic shapes
fn add_pieces(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, board_config: &Res<BoardConfig>, checkers_state: &Res<CheckersState>){
    let sq_dim: f32 = (board_config.world_dim - (board_config.border_size * 2.0)) / board_config.board_dim as f32;
    let scaled_sq_dim: f32 = board_config.piece_scale * sq_dim;

    for row in 0..board_config.board_dim {
        for col in 0..board_config.board_dim {
            match checkers_state.board[row][col] {    
                None => (),
                Some(piece) => {
                    let color = match piece.col {
                        PieceColor::Black => Color::rgb(0.25, 0.25, 0.25),
                        PieceColor::Red => Color::RED,
                    };

                    let position = compute_piece_center(row, col, board_config);

                    let parent = commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box{
                                                                min_x: - scaled_sq_dim / 2.0,
                                                                max_x: scaled_sq_dim / 2.0,
                                                                min_y: - board_config.piece_height / 2.0,
                                                                max_y: board_config.piece_height / 2.0,
                                                                min_z: - scaled_sq_dim / 2.0,
                                                                max_z: scaled_sq_dim / 2.0
                                        })),
                        material: materials.add(color.into()),
                        transform: Transform::from_xyz(position.x, position.y, position.z),
                        ..default()
                    }).insert(PieceComponent{row: row, col: col, color: piece.col, typ: piece.typ}).id();
                    if piece.typ == PieceType::King {
                        let child = commands.spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Box{
                                                                        min_x: - scaled_sq_dim / 2.0,
                                                                        max_x: scaled_sq_dim / 2.0,
                                                                        min_y: - board_config.piece_height / 2.0,
                                                                        max_y: (board_config.piece_height / 2.0) * 1.75,
                                                                        min_z: - scaled_sq_dim / 2.0,
                                                                        max_z: scaled_sq_dim / 2.0
                                                                })),
                            material: materials.add(color.into()),
                            transform: Transform::from_scale(Vec3{x: 0.8, y: 1.0, z: 0.8}),
                            ..default()
                        }).id();
                        commands.entity(parent).push_children(&[child]);
                    }
                }
            }
            
        }
    }
}

// Function to add board using basic shapes
fn add_board(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, board_config: &Res<BoardConfig>){
    let sq_dim: f32 = (board_config.world_dim - (board_config.border_size * 2.0)) / board_config.board_dim as f32;

    // draw top border
    let board = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box{
                                                min_x: board_config.offset_x,
                                                max_x: board_config.offset_x + board_config.world_dim,
                                                min_y: - board_config.board_height / 2.0,
                                                max_y: board_config.board_height / 2.0,
                                                min_z: board_config.offset_z,
                                                max_z: board_config.offset_z + board_config.border_size
                        })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::ONE * 1.0),
        ..default()
    }).insert(BoardComponent).id();

    // draw bottom border
    let child = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box{
                                                min_x: board_config.offset_x,
                                                max_x: board_config.offset_x + board_config.world_dim,
                                                min_y: - board_config.board_height / 2.0,
                                                max_y: board_config.board_height / 2.0,
                                                min_z: board_config.offset_z + board_config.world_dim - board_config.border_size,
                                                max_z: board_config.offset_z + board_config.world_dim
                        })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..default()
    }).id();
    commands.entity(board).push_children(&[child]);

    // draw left border
    let child = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box{
                                                min_x: board_config.offset_x,
                                                max_x: board_config.offset_x + board_config.border_size,
                                                min_y: - board_config.board_height / 2.0, 
                                                max_y: board_config.board_height / 2.0,
                                                min_z: board_config.offset_z,
                                                max_z: board_config.offset_z + board_config.world_dim
                        })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..default()
    }).id();
    commands.entity(board).push_children(&[child]);

    // draw right border
    let child = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box{
                                                min_x: board_config.offset_x + board_config.world_dim - board_config.border_size,
                                                max_x: board_config.offset_x + board_config.world_dim,
                                                min_y: - board_config.board_height / 2.0,
                                                max_y: board_config.board_height / 2.0,
                                                min_z: board_config.offset_z,
                                                max_z: board_config.offset_z + board_config.world_dim
                        })),
        material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
        ..default()
    }).id();
    commands.entity(board).push_children(&[child]);


    // draw the squares
    for z in 0..board_config.board_dim {
        for x in 0..board_config.board_dim {
            if (x + (z % 2)) % 2 == 0 {
                let sq_offset_x = board_config.offset_x + board_config.border_size + x as f32 * sq_dim;
                let sq_offset_z = board_config.offset_z + board_config.border_size + z as f32 * sq_dim;
                let child = commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box{
                                                            min_x: sq_offset_x,
                                                            max_x: sq_offset_x + sq_dim,
                                                            min_y: - board_config.board_height / 2.0,
                                                            max_y: board_config.board_height / 2.0,
                                                            min_z: sq_offset_z,
                                                            max_z: sq_offset_z + sq_dim
                                    })),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..default()
                }).insert(BoardSquareComponent{row: z, col: x}).id();
                commands.entity(board).push_children(&[child]);
            }
        }
    }
}
