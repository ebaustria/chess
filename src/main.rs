use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};

const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
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
        .run();
}

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

#[derive(Component, Debug, Clone, Copy)]
struct Position {
    col_label: ColLabel,
    row_label: u8,
}

#[derive(Component, Debug)]
struct Tile {
    position: Position,
}

#[derive(Component)]
struct Piece {
    name: String,
    position: Position,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    for row in 0..NUM_ROWS {
        for column in 0..NUM_COLUMNS {
            let offset: f32 = -(WINDOW_DIMENSION / 2.) + (TILE_SIZE.x / 2.);
            let tile_position = Vec2::new(
                offset + column as f32 * TILE_SIZE.x,
                offset + row as f32 * TILE_SIZE.y
            );

            let current_pos: Position = get_position(row, &column);
            // println!("Current position: {:?}", current_pos);
            // println!("Tile position: {:?}", tile_position);

            commands
                .spawn()
                .insert(Tile { position: current_pos })
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
                commands
                    .spawn()
                    .insert(Piece { name: String::from(name), position: current_pos })
                    .insert_bundle(SpriteBundle {
                        texture: asset_server.load(&path),
                        transform: Transform {
                            translation: tile_position.extend(0.0),
                            // scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    });
            }
        }
    }
}

fn get_piece_name(current_position: Position) -> &'static str {
    let Position { row_label, col_label} = current_position;

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

fn get_position(row: u8, column: &u8) -> Position {
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

    return Position {
        col_label: column_position,
        row_label: row + 1
    };
}
