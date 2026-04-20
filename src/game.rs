use crate::board::{
    check_bounds, default_king_data, get_pos_label, get_tile_color, index_for_pos, init_board,
    init_king_positions, update_king_data, Position, PositionLabel, Tile, BOARD_DIMENSION,
};
use crate::check::{check_checkmate, prevent_check};
use crate::pieces::{get_possible_moves_for_piece, init_piece_data, Team};
use crate::util::load_image;
use crate::{GameState, Light, Piece, Selected, HALF_TILE, NUM_COLUMNS, NUM_ROWS, TILE_SIZE};
use bevy::app::{App, FixedUpdate, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::color::Color;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Commands, Entity, MouseButton, OnEnter, Query, Res, ResMut, Resource, Sprite,
    Transform, Window, With, Without,
};
use bevy::prelude::{in_state, IntoScheduleConfigs, States};
use bevy::window::PrimaryWindow;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::process;

#[derive(Resource)]
pub struct ImageCache {
    pub(crate) white_pawn: Handle<Image>,
    pub(crate) white_knight: Handle<Image>,
    pub(crate) white_bishop: Handle<Image>,
    pub(crate) white_rook: Handle<Image>,
    pub(crate) white_queen: Handle<Image>,
    pub(crate) white_king: Handle<Image>,
    pub(crate) black_pawn: Handle<Image>,
    pub(crate) black_knight: Handle<Image>,
    pub(crate) black_bishop: Handle<Image>,
    pub(crate) black_rook: Handle<Image>,
    pub(crate) black_queen: Handle<Image>,
    pub(crate) black_king: Handle<Image>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameStatus {
    #[default]
    Splash,
    Menu,
    Game,
}

pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameStatus::Game),
        (load_sprites, setup_game).chain(),
    )
    .add_systems(
        FixedUpdate,
        (
            select_piece_system,
            cleanup_select_system,
            prevent_check_system,
            handle_move_system,
            enforce_checkmate_system,
        )
            .chain()
            .run_if(in_state(GameStatus::Game)),
    )
    .add_systems(Update, bevy::window::close_when_requested);
}

fn load_sprites(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ImageCache {
        white_pawn: load_image(asset_server.borrow(), "wP"),
        white_knight: load_image(asset_server.borrow(), "wN"),
        white_bishop: load_image(asset_server.borrow(), "wB"),
        white_rook: load_image(asset_server.borrow(), "wR"),
        white_queen: load_image(asset_server.borrow(), "wQ"),
        white_king: load_image(asset_server.borrow(), "wK"),
        black_pawn: load_image(asset_server.borrow(), "bP"),
        black_knight: load_image(asset_server.borrow(), "bN"),
        black_bishop: load_image(asset_server.borrow(), "bB"),
        black_rook: load_image(asset_server.borrow(), "bR"),
        black_queen: load_image(asset_server.borrow(), "bQ"),
        black_king: load_image(asset_server.borrow(), "bK"),
    });
}

fn setup_game(mut commands: Commands, image_cache: Res<ImageCache>) {
    let mut game_state = GameState {
        turn: Team::White,
        highlight_coords: Vec2::ZERO,
        selected_piece: None,
        board: init_board(),
        white_king_data: default_king_data(),
        black_king_data: default_king_data(),
    };

    // commands.spawn(Camera2d::default()).insert(MainCamera);

    for row in 0..NUM_ROWS {
        for column in 0..NUM_COLUMNS {
            let offset: f32 = -(BOARD_DIMENSION / 2.) + HALF_TILE;
            let tile_position = Vec2::new(
                offset + column as f32 * TILE_SIZE.x,
                offset + row as f32 * TILE_SIZE.y,
            );

            let (col_label, row_label) = get_pos_label(row, &column);
            let position_label = PositionLabel {
                col_label,
                row_label,
            };
            let current_pos = Position {
                position_label,
                coordinates: tile_position,
            };
            // println!("Current position: {:?}", current_pos);

            commands.spawn((
                Sprite {
                    color: get_tile_color(&row, &column),
                    ..default()
                },
                Transform {
                    translation: tile_position.extend(0.0),
                    scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                    ..default()
                },
            ));

            if !(2..=5).contains(&row) {
                let (handle, team, piece_type) = init_piece_data(image_cache.borrow(), current_pos);
                let piece_id: Entity = commands
                    .spawn((
                        Sprite::from_image(handle),
                        Transform::from_translation(tile_position.extend(999.0)),
                        Piece {
                            // name: name.to_string(),
                            position: current_pos,
                            team,
                            piece_type,
                            available_moves: Vec::new(),
                        },
                    ))
                    .id();

                init_king_positions(piece_type, team, &mut game_state, current_pos);
                game_state.board[row as usize][column as usize] = Tile {
                    position: current_pos,
                    team,
                    piece: Option::from(piece_id),
                };
            } else {
                game_state.board[row as usize][column as usize] = Tile {
                    position: current_pos,
                    team: Team::None,
                    piece: None,
                };
            }
        }
    }

    commands.insert_resource(game_state);
}

fn select_piece_system(
    buttons: Res<ButtonInput<MouseButton>>,
    query_windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut query_unselected: Query<(Entity, &mut Piece), Without<Selected>>,
    mut query_selected: Query<Entity, With<Selected>>,
) {
    let mouse_pos = query_windows.single().unwrap().cursor_position();

    if mouse_pos.is_none() {
        return;
    }

    if buttons.just_pressed(MouseButton::Left) {
        for (entity, mut piece) in query_unselected.iter_mut() {
            let piece_coords = piece.position.coordinates;
            let in_bounds: bool = check_bounds(piece_coords.x, piece_coords.y, mouse_pos.unwrap());
            if in_bounds
                && piece.team == game_state.turn
                && game_state.highlight_coords != piece_coords
            {
                commands.spawn((
                    (
                        Sprite {
                            color: Color::srgba(0.12, 1.0, 0.06, 0.7),
                            custom_size: Some(Vec2::new(TILE_SIZE.x, TILE_SIZE.y)),
                            ..default()
                        },
                        Transform::from_translation(piece_coords.extend(0.0)),
                    ),
                    Light {
                        coordinates: piece_coords,
                    },
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
    game_state: Res<GameState>,
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
            prevent_check(
                selected_piece.borrow_mut(),
                selected_entity,
                enemy_piece,
                king_pos,
                &game_state,
            );
        }
    }
}

fn enforce_checkmate_system(
    game_state: Res<GameState>,
    query_unselected: Query<(Entity, &mut Piece), Without<Selected>>,
) {
    if game_state.selected_piece.is_some() {
        return;
    }

    let is_checkmate: bool = if game_state.turn == Team::White {
        check_checkmate(
            game_state.turn,
            game_state.white_king_data.position,
            game_state.board,
            query_unselected,
        )
    } else {
        check_checkmate(
            game_state.turn,
            game_state.black_king_data.position,
            game_state.board,
            query_unselected,
        )
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
    mut game_state: ResMut<GameState>,
) {
    let mouse_pos = query_windows.single().unwrap().cursor_position();
    if mouse_pos.is_none()
        || !buttons.just_pressed(MouseButton::Left)
        || game_state.selected_piece.is_none()
    {
        return;
    }

    if let Ok((entity, mut piece, mut transform)) =
        query.get_mut(game_state.selected_piece.unwrap())
    {
        for position in &piece.available_moves {
            if check_bounds(
                position.coordinates.x,
                position.coordinates.y,
                mouse_pos.unwrap(),
            ) {
                let delta: Vec2 = Vec2::new(
                    position.coordinates.x - piece.position.coordinates.x,
                    position.coordinates.y - piece.position.coordinates.y,
                );
                transform.translation.x += delta.x;
                transform.translation.y += delta.y;
                transform.translation.z = 999f32;

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
                game_state.turn = if game_state.turn == Team::White {
                    Team::Black
                } else {
                    Team::White
                };

                piece.position = *position;
                piece.available_moves = Vec::new();
                commands.entity(entity).remove::<Selected>();
                break;
            }
        }
    }
}

fn cleanup_select_system(
    query: Query<(Entity, &mut Light)>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    for (entity, highlight) in query.iter() {
        if highlight.coordinates != game_state.highlight_coords {
            commands.entity(entity).despawn();
            break;
        }
    }
}
