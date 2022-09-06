use bevy::prelude::Color;
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use crate::{ColLabel, HALF_TILE, Piece, PieceType, Position, PositionLabel, Team, Tile};

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

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
    return TILE_DARK;
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

    return (column_position, row + 1);
}

pub fn get_piece_data(current_position: Position) -> (&'static str, Team, PieceType) {
    let Position { position_label, .. } = current_position;
    let PositionLabel { row_label, col_label } = position_label;

    if row_label == 7 {
        return ("bP", Team::BLACK, PieceType::PAWN);
    }

    if row_label == 2 {
        return ("wP", Team::WHITE, PieceType::PAWN);
    }

    if row_label == 8 {
        let data = match col_label {
            ColLabel::A => ("bR", Team::BLACK, PieceType::ROOK),
            ColLabel::B => ("bN", Team::BLACK, PieceType::KNIGHT),
            ColLabel::C => ("bB", Team::BLACK, PieceType::BISHOP),
            ColLabel::D => ("bQ", Team::BLACK, PieceType::QUEEN),
            ColLabel::E => ("bK", Team::BLACK, PieceType::KING),
            ColLabel::F => ("bB", Team::BLACK, PieceType::BISHOP),
            ColLabel::G => ("bN", Team::BLACK, PieceType::KNIGHT),
            ColLabel::H => ("bR", Team::BLACK, PieceType::ROOK)
        };
        return data;
    }

    let data = match col_label {
        ColLabel::A => ("wR", Team::WHITE, PieceType::ROOK),
        ColLabel::B => ("wN", Team::WHITE, PieceType::KNIGHT),
        ColLabel::C => ("wB", Team::WHITE, PieceType::BISHOP),
        ColLabel::D => ("wQ", Team::WHITE, PieceType::QUEEN),
        ColLabel::E => ("wK", Team::WHITE, PieceType::KING),
        ColLabel::F => ("wB", Team::WHITE, PieceType::BISHOP),
        ColLabel::G => ("wN", Team::WHITE, PieceType::KNIGHT),
        ColLabel::H => ("wR", Team::WHITE, PieceType::ROOK)
    };
    return data;
}

pub fn check_bounds(x_coord: f32, y_coord: f32, mouse_coords: Vec2) -> bool {
    let right_bound: f32 = x_coord + HALF_TILE;
    let left_bound: f32 = x_coord - HALF_TILE;
    let upper_bound: f32 = y_coord + HALF_TILE;
    let lower_bound: f32 = y_coord - HALF_TILE;

    if mouse_coords.x <= right_bound && mouse_coords.x >= left_bound && mouse_coords.y <= upper_bound && mouse_coords.y >= lower_bound {
        return true;
    }
    return false;
}

pub fn get_possible_moves_for_piece(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    return match piece.piece_type {
        PieceType::PAWN => possible_moves_for_pawn(piece, board),
        PieceType::BISHOP => Vec::new(),
        PieceType::KNIGHT => Vec::new(),
        PieceType::ROOK => Vec::new(),
        PieceType::QUEEN => Vec::new(),
        PieceType::KING => Vec::new(),
    };
}

fn possible_moves_for_pawn(piece: &Piece, board: &[[Tile; 8]; 8]) -> Vec<Position> {
    let mut result = Vec::new();
    if piece.team == Team::WHITE {
        let pos_label = piece.position.position_label;
        if pos_label.row_label == 2 {
            let row_label = pos_label.row_label + 1;
            if board[row_label as usize][pos_label.col_label as usize].team == Team::NONE {
                result.push(board[row_label as usize][pos_label.col_label as usize].position);
            }
        }

        if board[pos_label.row_label as usize][pos_label.col_label as usize].team == Team::NONE {
            result.push(board[pos_label.row_label as usize][pos_label.col_label as usize].position);
        }

        if board[pos_label.row_label as usize][(pos_label.col_label as u8 + 1) as usize].team == Team::BLACK {
            result.push(board[pos_label.row_label as usize][(pos_label.col_label as u8 + 1) as usize].position);
        }

        if board[pos_label.row_label as usize][(pos_label.col_label as u8 - 1) as usize].team == Team::BLACK {
            result.push(board[pos_label.row_label as usize][(pos_label.col_label as u8 - 1) as usize].position);
        }
    }
    return result;
}