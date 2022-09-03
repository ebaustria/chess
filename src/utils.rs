use bevy::prelude::Color;
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use crate::{HALF_TILE, Piece, PieceType, Position, PositionLabel};

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColLabel {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
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

pub fn get_piece_data(current_position: Position) -> (&'static str, PieceType) {
    let Position { position_label, .. } = current_position;
    let PositionLabel { row_label, col_label } = position_label;

    if row_label == 7 {
        return ("bP", PieceType::PAWN);
    }

    if row_label == 2 {
        return ("wP", PieceType::PAWN);
    }

    if row_label == 8 {
        let data = match col_label {
            ColLabel::A => ("bR", PieceType::ROOK),
            ColLabel::B => ("bN", PieceType::KNIGHT),
            ColLabel::C => ("bB", PieceType::BISHOP),
            ColLabel::D => ("bQ", PieceType::QUEEN),
            ColLabel::E => ("bK", PieceType::KING),
            ColLabel::F => ("bB", PieceType::BISHOP),
            ColLabel::G => ("bN", PieceType::KNIGHT),
            ColLabel::H => ("bR", PieceType::ROOK)
        };
        return data;
    }

    let data = match col_label {
        ColLabel::A => ("wR", PieceType::ROOK),
        ColLabel::B => ("wN", PieceType::KNIGHT),
        ColLabel::C => ("wB", PieceType::BISHOP),
        ColLabel::D => ("wQ", PieceType::QUEEN),
        ColLabel::E => ("wK", PieceType::KING),
        ColLabel::F => ("wB", PieceType::BISHOP),
        ColLabel::G => ("wN", PieceType::KNIGHT),
        ColLabel::H => ("wR", PieceType::ROOK)
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

pub fn get_possible_moves_for_piece(piece: Piece) -> Vec<PositionLabel> {
    return match piece.piece_type {
        PieceType::PAWN => Vec::new(),
        PieceType::BISHOP => Vec::new(),
        PieceType::KNIGHT => Vec::new(),
        PieceType::ROOK => Vec::new(),
        PieceType::QUEEN => Vec::new(),
        PieceType::KING => Vec::new(),
    };
}