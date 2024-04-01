use std::borrow::BorrowMut;
use std::process;
use bevy::{prelude::*, window::PresentMode};
use bevy::utils::petgraph::visit::Walker;
use bevy::window::{PrimaryWindow, WindowTheme};
use crate::board::{BOARD_DIMENSION, check_bounds, ColLabel, default_king_data, get_pos_label, get_tile_color, index_for_pos, init_board, init_king_positions, Position, PositionLabel, simulate_move, Tile, update_king_data};
use crate::check::{check_checkmate, prevent_check};
use crate::pieces::{init_piece_data, get_possible_moves_for_piece, PieceType, Team, KingData};

const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
const HALF_TILE: f32 = TILE_SIZE.x / 2.;
const NUM_ROWS: u8 = 8;
const NUM_COLUMNS: u8 = 8;

mod pieces;
mod board;
mod check;

fn main() {
    App::new()
        // .insert_resource(Window {
        //     title: "Chess".to_string(),
        //     width: WINDOW_DIMENSION,
        //     height: WINDOW_DIMENSION,
        //     present_mode: PresentMode::Fifo,
        //     ..default()
        // })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chess".into(),
                    name: Some("chess.app".into()),
                    resolution: (BOARD_DIMENSION, BOARD_DIMENSION).into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            })
         ))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                // update_mouse_coords_system,
                select_piece_system,
                cleanup_select_system,
                prevent_check_system,
                handle_move_system,
                enforce_checkmate_system
            ).chain()
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
pub struct Selected;

#[derive(Resource)]
struct Mouse {
    coords: Vec2,
}

#[derive(Component)]
struct Light {
    coordinates: Vec2,
}

#[derive(Component, Debug)]
pub struct Piece {
    // name: String,
    position: Position,
    piece_type: PieceType,
    team: Team,
    available_moves: Vec<Position>,
}

#[derive(Resource)]
pub struct GameState {
    turn: Team,
    highlight_coords: Vec2,
    selected_piece: Option<Entity>,
    board: [[Tile; 8]; 8],
    white_king_data: KingData,
    black_king_data: KingData,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Mouse {
        coords: Vec2::new(280., 280.),
    });

    let mut game_state = GameState {
        turn: Team::White,
        highlight_coords: Vec2::ZERO,
        selected_piece: None,
        board: init_board(),
        white_king_data: default_king_data(),
        black_king_data: default_king_data(),
    };

    commands.spawn(Camera2dBundle::default()).insert(MainCamera);

    for row in 0..NUM_ROWS {
        for column in 0..NUM_COLUMNS {
            let offset: f32 = -(BOARD_DIMENSION / 2.) + HALF_TILE;
            let tile_position = Vec2::new(
                offset + column as f32 * TILE_SIZE.x,
                offset + row as f32 * TILE_SIZE.y
            );


            let (col_label, row_label) = get_pos_label(row, &column);
            let position_label = PositionLabel { col_label, row_label };
            let current_pos = Position { position_label, coordinates: tile_position };
            // println!("Current position: {:?}", current_pos);

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: get_tile_color(&row, &column),
                    ..default()
                },
                transform: Transform {
                    translation: tile_position.extend(0.0),
                    scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                    ..default()
                },
                ..default()
            });

            if !(2..=5).contains(&row) {
                let (name, team, piece_type) = init_piece_data(current_pos);
                let path = format!("../assets/pieces/{}.png", name);
                let piece_id: Entity = commands
                    .spawn((
                        SpriteBundle {
                            texture: asset_server.load(&path),
                            transform: Transform {
                                translation: tile_position.extend(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        Piece {
                            // name: name.to_string(),
                            position: current_pos,
                            team,
                            piece_type,
                            available_moves: Vec::new()
                        }
                    )).id();

                init_king_positions(piece_type, team, &mut game_state, current_pos);
                game_state.board[row as usize][column as usize] = Tile { position: current_pos, team, piece: Option::from(piece_id) };
            } else {
                game_state.board[row as usize][column as usize] = Tile { position: current_pos, team: Team::None, piece: None, };
            }
        }
    }

    commands.insert_resource(game_state);
}

fn select_piece_system(
    buttons: Res<ButtonInput<MouseButton>>,
    // mouse_coords: Res<Mouse>,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut query_unselected: Query<(Entity, &mut Piece), Without<Selected>>,
    mut query_selected: Query<Entity, With<Selected>>,
) {
    let mouse_pos = query_windows.single().cursor_position();

    if mouse_pos.is_none() {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        for (entity, mut piece) in query_unselected.iter_mut() {
            let piece_coords = piece.position.coordinates;
            let in_bounds: bool = check_bounds(piece_coords.x, piece_coords.y, mouse_pos.unwrap());
            if in_bounds && piece.team == game_state.turn && game_state.highlight_coords != piece_coords {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.12, 1.0, 0.06, 0.7),
                            ..default()
                        },
                        transform: Transform {
                            translation: piece_coords.extend(0.0),
                            scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Light { coordinates: piece_coords }
                ));
                game_state.highlight_coords = piece_coords;
                game_state.selected_piece = Option::from(entity);
                for ent in query_selected.iter_mut() {
                    commands.entity(ent).remove::<Selected>();
                }
                commands.entity(entity).insert(Selected);
                piece.available_moves = get_possible_moves_for_piece(&piece, &game_state.board);
                break;
            }
        }
    }
}

fn prevent_check_system(
    mut query_selected: Query<&mut Piece, With<Selected>>,
    query_unselected: Query<&mut Piece, Without<Selected>>,
    game_state: Res<GameState>
) {
    if game_state.selected_piece.is_none() {
        return;
    }

    let selected_entity: Entity = game_state.selected_piece.unwrap();
    if let Ok(mut selected_piece) = query_selected.get_mut(selected_entity) {
        let king_pos: Position = if game_state.turn == Team::White {
            game_state.white_king_data.position
        } else {
            game_state.black_king_data.position
        };
        for enemy_piece in query_unselected.iter() {
            prevent_check(selected_piece.borrow_mut(), selected_entity, enemy_piece, king_pos, &game_state);
        }
    }
}

fn enforce_checkmate_system(game_state: Res<GameState>, query_unselected: Query<(Entity, &mut Piece), Without<Selected>>) {
    let is_checkmate: bool = if game_state.turn == Team::White {
        check_checkmate(game_state.turn, game_state.white_king_data.position, game_state.board, query_unselected)
    } else {
        check_checkmate(game_state.turn, game_state.black_king_data.position, game_state.board, query_unselected)
    };

    if is_checkmate {
        println!("Checkmate!");
        process::exit(0x00);
    };
}

fn handle_move_system(
    buttons: Res<ButtonInput<MouseButton>>,
    // mouse_coords: Res<Mouse>,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Piece, &mut Transform), With<Selected>>,
    mut game_state: ResMut<GameState>
) {
    let mouse_pos = query_windows.single().cursor_position();
    if mouse_pos.is_none() || !buttons.just_pressed(MouseButton::Left) || game_state.selected_piece.is_none() {
        return;
    }

    if let Ok((entity, mut piece, mut transform)) = query.get_mut(game_state.selected_piece.unwrap()) {
        for position in &piece.available_moves {
            if check_bounds(position.coordinates.x, position.coordinates.y, mouse_pos.unwrap()) {
                println!("CLICKED POSITION: {}", position.coordinates);
                println!("PIECE POSITION: {}", piece.position.coordinates);
                let delta: Vec2 = Vec2::new(position.coordinates.x - piece.position.coordinates.x, position.coordinates.y - piece.position.coordinates.y);
                println!("DELTA: {}", delta);
                transform.translation.x += delta.x;
                transform.translation.y += delta.y;
                transform.translation.z = 0f32;

                let (old_row, old_col) = index_for_pos(piece.position.position_label);
                let (new_row, new_col) = index_for_pos(position.position_label);
                let new_pos: Position = game_state.board[new_row][new_col].position;
                update_king_data(&piece, &mut game_state, new_pos);
                let new_tile: &mut Tile = &mut game_state.board[new_row][new_col];

                // capture piece if tile contains enemy
                if new_tile.piece.is_some() {
                    commands.entity(new_tile.piece.unwrap()).despawn();
                }

                new_tile.team = piece.team;
                new_tile.piece = Option::from(entity);

                game_state.board[old_row][old_col].team = Team::None;
                game_state.board[old_row][old_col].piece = None;

                game_state.highlight_coords = Vec2::ZERO;
                game_state.selected_piece = None;
                game_state.turn = if game_state.turn == Team::White { Team::Black } else { Team::White };

                piece.position = *position;
                piece.available_moves = Vec::new();
                commands.entity(entity).remove::<Selected>();
                break;
            }
        }
    }
}

fn cleanup_select_system(query: Query<(Entity, &mut Light)>, mut commands: Commands, game_state: Res<GameState>) {
    for (entity, highlight) in query.iter() {
        if highlight.coordinates != game_state.highlight_coords {
            commands.entity(entity).despawn();
            break;
        }
    }
}

// fn update_mouse_coords_system(
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mut mouse_coords: ResMut<Mouse>,
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
// ) {
//     for event in cursor_moved_events.read() {
//         println!("MOUSE COORDS UNTRANSFORMED: {}", event.position);
//         let (camera, camera_transform) = q_camera.single();
//
//         // get the size of the window
//         let window_size = Vec2::new(BOARD_DIMENSION, BOARD_DIMENSION);
//
//         // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
//         let ndc = (event.position / window_size) * 2.0 - Vec2::ONE;
//
//         // matrix for undoing the projection and camera transform
//         let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
//
//         // use it to convert ndc to world-space coordinates
//         let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
//
//         // reduce it to a 2D value
//         mouse_coords.coords = world_pos.truncate();
//         println!("MOUSE COORDS: {}", mouse_coords.coords);
//     }
// }
