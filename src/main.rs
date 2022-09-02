use bevy::{prelude::*, window::PresentMode};
use crate::utils::*;

const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
const HALF_TILE: f32 = TILE_SIZE.x / 2.;
const WINDOW_DIMENSION: f32 = 640.0;
const NUM_ROWS: u8 = 8;
const NUM_COLUMNS: u8 = 8;

pub mod utils;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chess".to_string(),
            width: WINDOW_DIMENSION,
            height: WINDOW_DIMENSION,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_mouse_coords_system)
        .add_system(select_piece_system)
        .add_system(cleanup_select_system)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component, Debug, PartialEq)]
enum Team {
    WHITE,
    BLACK
}

struct Mouse {
    coords: Vec2,
}

#[derive(Component)]
struct Light {
    coordinates: Vec2,
}

struct GameState {
    turn: Team,
    highlight_coords: Vec2,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    col_label: ColLabel,
    row_label: u8,
    coordinates: Vec2,
}

#[derive(Component, Debug)]
struct Tile {
    position: Position,
}

#[derive(Component)]
struct Piece {
    name: String,
    position: Position,
    team: Team,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Mouse {
        coords: Vec2::new(280., 280.),
    });

    commands.insert_resource(GameState {
        turn: Team::WHITE,
        highlight_coords: Vec2::ZERO,
    });

    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);

    for row in 0..NUM_ROWS {
        for column in 0..NUM_COLUMNS {
            let offset: f32 = -(WINDOW_DIMENSION / 2.) + HALF_TILE;
            let tile_position = Vec2::new(
                offset + column as f32 * TILE_SIZE.x,
                offset + row as f32 * TILE_SIZE.y
            );


            let (col_label, row_label) = get_pos_label(row, &column);
            let current_pos: Position = Position { col_label, row_label, coordinates: tile_position };
            // println!("Current position: {:?}", current_pos);
            // println!("Tile position: {:?}", tile_position);

            commands
                .spawn()
                .insert(Tile { position: current_pos, })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: get_tile_color(&row, &column),
                        ..default()
                    },
                    transform: Transform {
                        translation: tile_position.extend(0.0),
                        scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                        ..default()
                   },
                   ..default()
                });

            if row > 5 || row < 2 {
                let name = get_piece_name(current_pos);
                let path = format!("../assets/pieces/{}.png", name);
                let team: Team = if name.contains("w") { Team::WHITE } else { Team::BLACK };
                commands
                    .spawn()
                    .insert(Piece { name: String::from(name), position: current_pos, team })
                    .insert_bundle(SpriteBundle {
                        texture: asset_server.load(&path),
                        transform: Transform {
                            translation: tile_position.extend(0.0),
                            ..default()
                        },
                        ..default()
                    });
            }
        }
    }
}

fn select_piece_system(
    buttons: Res<Input<MouseButton>>,
    mouse_coords: Res<Mouse>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut query: Query<&mut Piece>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for piece in query.iter_mut() {
            let piece_coords = piece.position.coordinates;
            if piece.team == game_state.turn && check_bounds(piece_coords.x, piece_coords.y, mouse_coords.coords) {
                commands
                    .spawn()
                    .insert(Light { coordinates: piece_coords })
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.12, 1.0, 0.06, 0.7),
                            ..default()
                        },
                        transform: Transform {
                            translation: piece_coords.extend(0.0),
                            scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                game_state.highlight_coords = piece_coords;
                break;
            }
        }
    }
}

fn cleanup_select_system(query: Query<(Entity, &mut Light)>, mut commands: Commands, game_state: Res<GameState>) {
    for (entity, highlight) in query.iter() {
        if highlight.coordinates != game_state.highlight_coords {
            commands.entity(entity).despawn();
            break;
        }
    }
}

fn update_mouse_coords_system(
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_coords: ResMut<Mouse>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    for event in cursor_moved_events.iter() {
        let (camera, camera_transform) = q_camera.single();

        // get the size of the window
        let window_size = Vec2::new(WINDOW_DIMENSION, WINDOW_DIMENSION);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (event.position / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        mouse_coords.coords = world_pos.truncate();
    }
}
