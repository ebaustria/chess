use bevy::prelude::{ Color, Vec2 };
use bevy::ecs::component::Component;
use crate::{Entity, GameState, get_possible_moves_for_piece, HALF_TILE, KingData, Piece, PieceType, Team};

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub(crate) team: Team,
    pub(crate) position: Position,
    pub(crate) piece: Option<Entity>,
}

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColLabel {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct PositionLabel {
    pub(crate) col_label: ColLabel,
    pub(crate) row_label: u8,
}

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub struct Position {
    pub(crate) position_label: PositionLabel,
    pub(crate) coordinates: Vec2,
}

pub fn init_board() -> [[Tile; 8]; 8] {
    [[
        Tile {
            team: Team::None,
            position: Position {
                position_label: PositionLabel { col_label: ColLabel::A, row_label: 1 },
                coordinates: Vec2::ZERO
            },
            piece: None,
        };
    8]; 8]
}

pub fn default_king_data() -> KingData {
    KingData {
        position: Position {
            position_label: PositionLabel { col_label: ColLabel::A, row_label: 0 },
            coordinates: Vec2::ZERO
        },
        available_moves: Vec::new(),
    }
}

pub fn init_king_positions(piece_type: PieceType, team: Team, game_state: &mut GameState, pos: Position) -> bool {
    if piece_type == PieceType::KING {
        if team == Team::White {
            game_state.white_king_data.position = pos;
        } else {
            game_state.black_king_data.position = pos;
        }
        return true;
    }
    false
}

pub fn update_king_data(piece: &Piece, game_state: &mut GameState, pos: Position) {
    if init_king_positions(piece.piece_type, piece.team, game_state, pos) {
        if piece.team == Team::White {
            game_state.white_king_data.available_moves = get_possible_moves_for_piece(piece, &game_state.board);
        } else {
            game_state.black_king_data.available_moves = get_possible_moves_for_piece(piece, &game_state.board);
        }
    }
}

pub fn get_tile_color(row: &u8, column: &u8) -> Color {
    if row % 2 == 0 {
        if column % 2 == 0 {
            return TILE_DARK;
        }
        return TILE_LIGHT;
    }
    if column % 2 == 0 {
        return TILE_LIGHT;
    }
    TILE_DARK
}

pub fn index_for_pos(pos_label: PositionLabel) -> (usize, usize) {
    ((pos_label.row_label - 1 ) as usize, pos_label.col_label as usize)
}

pub fn get_pos_label(row: u8, column: &u8) -> (ColLabel, u8) {
    let column_position: ColLabel = match column {
        0 => ColLabel::A,
        1 => ColLabel::B,
        2 => ColLabel::C,
        3 => ColLabel::D,
        4 => ColLabel::E,
        5 => ColLabel::F,
        6 => ColLabel::G,
        _ => ColLabel::H,
    };

    (column_position, row + 1)
}

pub fn check_bounds(x_coord: f32, y_coord: f32, mouse_coords: Vec2) -> bool {
    let right_bound: f32 = x_coord + HALF_TILE;
    let left_bound: f32 = x_coord - HALF_TILE;
    let upper_bound: f32 = y_coord + HALF_TILE;
    let lower_bound: f32 = y_coord - HALF_TILE;

    if mouse_coords.x <= right_bound && mouse_coords.x >= left_bound && mouse_coords.y <= upper_bound && mouse_coords.y >= lower_bound {
        return true;
    }
    false
}

pub fn simulate_move(
    board: &mut [[Tile; 8]; 8],
    entity: Entity,
    team: Team,
    piece_pos: PositionLabel,
    goal_pos: PositionLabel
) {
    let (old_row, old_col) = index_for_pos(piece_pos);
    let (new_row, new_col) = index_for_pos(goal_pos);
    let new_tile:  &mut Tile = &mut board[new_row][new_col];

    new_tile.team = team;
    new_tile.piece = Option::from(entity);

    board[old_row][old_col].team = Team::None;
    board[old_row][old_col].piece = None;
}
