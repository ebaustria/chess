use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};

const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
const HALF_TILE: f32 = TILE_SIZE.x / 2.;
const WINDOW_DIMENSION: f32 = 640.0;
const NUM_ROWS: u8 = 8;
const NUM_COLUMNS: u8 = 8;
const TILE_LIGHT: Color = Color::BEIGE;
const TILE_DARK: Color = Color::OLIVE;

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
        .add_system(cleanup_highlight_system)
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component, Debug, Clone, Copy)]
enum ColLabel {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

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
struct Position {
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
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
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

/*
fn highlight_system(
    mut query: Query<&mut Tile>,
    mut commands: Commands,
    mouse_coords: Res<Mouse>,
) {
    info!("{:?}", mouse_coords.coords);
    let half_tile = TILE_SIZE.x / 2.;
    let coords = mouse_coords.coords;
    for mut tile in query.iter_mut() {
        let right_bound: f32 = tile.coordinates.x + half_tile;
        let left_bound: f32 = tile.coordinates.x - half_tile;
        let upper_bound: f32 = tile.coordinates.y + half_tile;
        let lower_bound: f32 = tile.coordinates.y - half_tile;
        // if coords <= right_bound && coords >= left_bound &&
    }
}
 */

fn check_bounds(x_coord: f32, y_coord: f32, mouse_coords: Vec2) -> bool {
    let right_bound: f32 = x_coord + HALF_TILE;
    let left_bound: f32 = x_coord - HALF_TILE;
    let upper_bound: f32 = y_coord + HALF_TILE;
    let lower_bound: f32 = y_coord - HALF_TILE;

    if mouse_coords.x <= right_bound && mouse_coords.x >= left_bound && mouse_coords.y <= upper_bound && mouse_coords.y >= lower_bound {
        return true;
    }
    return false;
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

fn cleanup_highlight_system(mut query: Query<(Entity, &mut Light)>, mut commands: Commands, mut game_state: ResMut<GameState>) {
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

fn get_piece_name(current_position: Position) -> &'static str {
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

fn get_tile_color(row: &u8, column: &u8) -> Color {
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

fn get_pos_label(row: u8, column: &u8) -> (ColLabel, u8) {
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
