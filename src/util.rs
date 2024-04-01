use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Image, Res};
use crate::board::BOARD_DIMENSION;

pub fn load_image(asset_server: &Res<AssetServer>, piece: &str) -> Handle<Image> {
    asset_server.load(format!("../assets/pieces/{}.png", piece))
}

pub fn transform_mouse_coords(mouse_coords: Vec2) -> Vec2 {
    let half_board: f32 = BOARD_DIMENSION * 0.5;
    Vec2::from((mouse_coords.x - half_board, -(mouse_coords.y - half_board)))
}
