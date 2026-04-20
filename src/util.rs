use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::asset::{AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Image, Res};

pub fn load_image(asset_server: &Res<AssetServer>, piece: &str) -> Handle<Image> {
    asset_server.load(format!("pieces/{}.png", piece))
}

pub fn transform_mouse_coords(mouse_coords: Vec2) -> Vec2 {
    let half_width: f32 = WINDOW_WIDTH as f32 * 0.5;
    let half_height: f32 = WINDOW_HEIGHT as f32 * 0.5;
    Vec2::from((mouse_coords.x - half_width, -(mouse_coords.y - half_height)))
}
