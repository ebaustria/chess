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

#[derive(Component)]
struct Tile;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());

    for row in 0..NUM_ROWS {
        for column in 0..NUM_COLUMNS {
            let offset: f32 = -(WINDOW_DIMENSION / 2.) + (TILE_SIZE.x / 2.);
            let tile_position = Vec2::new(
                offset + column as f32 * TILE_SIZE.x,
                offset + row as f32 * TILE_SIZE.y
            );

            commands
                .spawn()
                .insert(Tile)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: get_tile_color(row, column),
                        ..default()
                    },
                    transform: Transform {
                        translation: tile_position.extend(0.0),
                        scale: Vec3::new(TILE_SIZE.x, TILE_SIZE.y, 1.0),
                        ..default()
                   },
                   ..default()
                });
        }
    }
}

fn get_tile_color(row: u8, column: u8) -> Color {
    if row % 2 == 0 {
        if column % 2 == 0 {
            return TILE_LIGHT;
        }
        return TILE_DARK;
    }
    if column % 2 == 0 {
        return TILE_DARK;
    }
    return TILE_LIGHT;
}
