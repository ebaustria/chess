use bevy::prelude::Color;
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use std::collections::HashMap;
use crate::{HALF_TILE, Piece, PieceType, Position, PositionLabel, Team};

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColLabel {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8
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

pub fn get_possible_moves_for_piece(piece: &Piece, board: &HashMap<PositionLabel, Piece>) -> Vec<PositionLabel> {
    return match piece.piece_type {
        PieceType::PAWN => possible_moves_for_pawn(piece, board),
        PieceType::BISHOP => Vec::new(),
        PieceType::KNIGHT => Vec::new(),
        PieceType::ROOK => Vec::new(),
        PieceType::QUEEN => Vec::new(),
        PieceType::KING => Vec::new(),
    };
}

fn possible_moves_for_pawn(piece: &Piece, board: &HashMap<PositionLabel, Piece>) -> Vec<PositionLabel> {
    let mut result = Vec::new();
    if piece.team == Team::WHITE {
        let pos_label = piece.position.position_label;
        // result.push(PositionLabel { col_label: pos_label.col_label, row_label: pos_label.row_label - 1 });
        if piece.position.position_label.row_label == 2 {
            println!("ROW TWO");
            let forward_two = PositionLabel { col_label: pos_label.col_label, row_label: pos_label.row_label - 2 };
            /*
            if board.get(&forward_two) != None {
                println!("CONTAINS KEY!");
            }
             */
        }
    }
    return result;
}