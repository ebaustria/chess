use bevy::prelude::Color;
use bevy::ecs::component::Component;
use bevy::prelude::Vec2;
use crate::{HALF_TILE, Position};

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

#[derive(Component, Debug, Clone, Copy)]
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

pub fn get_piece_name(current_position: Position) -> &'static str {
    let Position { row_label, col_label, .. } = current_position;

    if row_label == 7 {
        return "bP";
    }

    if row_label == 2 {
        return "wP";
    }

    if row_label == 8 {
        let name = match col_label {
            ColLabel::A => "bR",
            ColLabel::B => "bN",
            ColLabel::C => "bB",
            ColLabel::D => "bQ",
            ColLabel::E => "bK",
            ColLabel::F => "bB",
            ColLabel::G => "bN",
            ColLabel::H => "bR"
        };
        return name;
    }

    let name = match col_label {
        ColLabel::A => "wR",
        ColLabel::B => "wN",
        ColLabel::C => "wB",
        ColLabel::D => "wQ",
        ColLabel::E => "wK",
        ColLabel::F => "wB",
        ColLabel::G => "wN",
        ColLabel::H => "wR"
    };
    return name;
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