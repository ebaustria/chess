use crate::util::transform_mouse_coords;
use crate::{get_possible_moves_for_piece, Entity, GameState, KingData, Piece, PieceType, Team};
use bevy::color::palettes::css;
use bevy::ecs::component::Component;
use bevy::prelude::{Color, Vec2};

pub(crate) const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
pub(crate) const HALF_TILE: f32 = TILE_SIZE.x / 2.;
pub(crate) const NUM_ROWS: u8 = 8;
pub(crate) const NUM_COLUMNS: u8 = 8;
const TILE_LIGHT: Color = Color::Srgba(css::BEIGE);
const TILE_DARK: Color = Color::Srgba(css::OLIVE);
pub const BOARD_DIMENSION: f32 = 640.0;

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
    H = 7,
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

// TODO Change these to default trait
pub fn init_board() -> [[Tile; 8]; 8] {
    [[Tile {
        team: Team::None,
        position: Position {
            position_label: PositionLabel {
                col_label: ColLabel::A,
                row_label: 1,
            },
            coordinates: Vec2::ZERO,
        },
        piece: None,
    }; 8]; 8]
}

pub fn default_king_data() -> KingData {
    KingData {
        position: Position {
            position_label: PositionLabel {
                col_label: ColLabel::A,
                row_label: 0,
            },
            coordinates: Vec2::ZERO,
        },
        available_moves: Vec::new(),
    }
}

pub fn init_king_positions(
    piece_type: PieceType,
    team: Team,
    game_state: &mut GameState,
    pos: Position,
) -> bool {
    if piece_type == PieceType::King {
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
            game_state.white_king_data.available_moves =
                get_possible_moves_for_piece(piece, &game_state.board);
        } else {
            game_state.black_king_data.available_moves =
                get_possible_moves_for_piece(piece, &game_state.board);
        }
    }
}

pub fn get_tile_color(row: &u8, column: &u8) -> Color {
    if row.is_multiple_of(2) {
        if column.is_multiple_of(2) {
            return TILE_DARK;
        }
        return TILE_LIGHT;
    }
    if column.is_multiple_of(2) {
        return TILE_LIGHT;
    }
    TILE_DARK
}

pub fn index_for_pos(pos_label: PositionLabel) -> (usize, usize) {
    (
        (pos_label.row_label - 1) as usize,
        pos_label.col_label as usize,
    )
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

    let transformed_coords = transform_mouse_coords(mouse_coords);

    if transformed_coords.x <= right_bound
        && transformed_coords.x >= left_bound
        && transformed_coords.y <= upper_bound
        && transformed_coords.y >= lower_bound
    {
        return true;
    }
    false
}

pub fn simulate_move(
    board: &mut [[Tile; 8]; 8],
    entity: Entity,
    team: Team,
    piece_pos: PositionLabel,
    goal_pos: PositionLabel,
) {
    let (old_row, old_col) = index_for_pos(piece_pos);
    let (new_row, new_col) = index_for_pos(goal_pos);
    let new_tile: &mut Tile = &mut board[new_row][new_col];

    new_tile.team = team;
    new_tile.piece = Option::from(entity);

    board[old_row][old_col].team = Team::None;
    board[old_row][old_col].piece = None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_board() {
        let board = init_board();
        let pos_placeholder = Position {
            position_label: PositionLabel {
                col_label: ColLabel::A,
                row_label: 1,
            },
            coordinates: Vec2::ZERO,
        };
        for row in board {
            for tile in row {
                assert_eq!(tile.team, Team::None);
                assert_eq!(tile.piece, None);
                assert_eq!(tile.position, pos_placeholder);
            }
        }
    }

    #[test]
    fn test_default_king_data() {
        let king_data = default_king_data();
        let pos_placeholder = Position {
            position_label: PositionLabel {
                col_label: ColLabel::A,
                row_label: 0,
            },
            coordinates: Vec2::ZERO,
        };
        assert_eq!(king_data.position, pos_placeholder);
        assert_eq!(king_data.available_moves, Vec::new());
    }

    #[test]
    fn test_get_tile_color_dark() {
        assert_eq!(get_tile_color(&2, &4), TILE_DARK);
        assert_eq!(get_tile_color(&3, &5), TILE_DARK);
    }

    #[test]
    fn test_get_tile_color_light() {
        assert_eq!(get_tile_color(&2, &3), TILE_LIGHT);
        assert_eq!(get_tile_color(&5, &4), TILE_LIGHT);
    }

    #[test]
    fn test_index_for_pos() {
        let mut pos_label = PositionLabel {
            col_label: ColLabel::A,
            row_label: 1,
        };

        assert_eq!(index_for_pos(pos_label), (0, 0));

        pos_label.col_label = ColLabel::B;
        pos_label.row_label = 4;

        assert_eq!(index_for_pos(pos_label), (3, 1))
    }

    #[test]
    fn test_get_pos_label() {
        assert_eq!(get_pos_label(4, &0), (ColLabel::A, 5));
        assert_eq!(get_pos_label(6, &1), (ColLabel::B, 7));
        assert_eq!(get_pos_label(3, &2), (ColLabel::C, 4));
        assert_eq!(get_pos_label(4, &3), (ColLabel::D, 5));
        assert_eq!(get_pos_label(0, &4), (ColLabel::E, 1));
        assert_eq!(get_pos_label(1, &5), (ColLabel::F, 2));
        assert_eq!(get_pos_label(5, &6), (ColLabel::G, 6));
        assert_eq!(get_pos_label(2, &7), (ColLabel::H, 3));
    }

    #[test]
    fn test_check_bounds() {
        let x_coord: f32 = 40.;
        let y_coord: f32 = -40.;

        let in_bounds_mouse_coords = Vec2::new(580., 400.);
        assert!(check_bounds(x_coord, y_coord, in_bounds_mouse_coords));

        let out_of_bounds_mouse_coords = Vec2::new(580., 359.);
        assert_eq!(check_bounds(x_coord, y_coord, out_of_bounds_mouse_coords), false);
    }
}