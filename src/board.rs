use bevy::prelude::{ Color, Vec2 };
use bevy::ecs::component::Component;
use crate::{ HALF_TILE };

const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

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

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub(crate) position_label: PositionLabel,
    pub(crate) coordinates: Vec2,
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

pub fn index_for_pos(pos_label: PositionLabel) -> (usize, usize) {
    return ((pos_label.row_label - 1 ) as usize, pos_label.col_label as usize);
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
