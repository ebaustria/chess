use crate::board::{simulate_move, ColLabel, Position, PositionLabel, Tile};
use crate::pieces::{get_possible_moves_for_piece, KingData, PieceType, Team};
use bevy::window::{WindowResolution, WindowTheme};
use bevy::{prelude::*, window::PresentMode};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const WINDOW_WIDTH: u32 = 1080;
const WINDOW_HEIGHT: u32 = 720;

const TILE_SIZE: Vec2 = Vec2::new(80., 80.);
const HALF_TILE: f32 = TILE_SIZE.x / 2.;
const NUM_ROWS: u8 = 8;
const NUM_COLUMNS: u8 = 8;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
struct Light {
    coordinates: Vec2,
}

#[derive(Component, Debug)]
pub struct Piece {
    // name: String,
    position: Position,
    piece_type: PieceType,
    team: Team,
    available_moves: Vec<Position>,
}

#[derive(Resource)]
pub struct GameState {
    turn: Team,
    highlight_coords: Vec2,
    selected_piece: Option<Entity>,
    board: [[Tile; 8]; 8],
    white_king_data: KingData,
    black_king_data: KingData,
}

mod board;
mod check;
mod game;
mod pieces;
mod util;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chess".into(),
                name: Some("chess.app".into()),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .init_state::<game::GameStatus>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

mod splash {
    use bevy::prelude::*;

    use super::game::GameStatus;

    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameStatus::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameStatus::Splash)));
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("pieces/bP.png");
        commands.spawn((
            DespawnOnExit(GameStatus::Splash),
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            OnSplashScreen,
            children![(
                ImageNode::new(icon),
                Node {
                    width: px(200),
                    ..default()
                },
            )],
        ));
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameStatus>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameStatus::Menu);
        }
    }
}

mod menu {
    use bevy::{app::AppExit, color::palettes::css::CRIMSON, ecs::spawn::SpawnIter, prelude::*};

    use super::{game::GameStatus, TEXT_COLOR};

    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameStatus::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameStatus::Menu)),
            );
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let button_node = Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_node = Node {
            width: px(30),
            position_type: PositionType::Absolute,
            left: px(10),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        let right_icon = asset_server.load("pieces/bB.png");
        let wrench_icon = asset_server.load("pieces/wK.png");
        let exit_icon = asset_server.load("pieces/wR.png");

        commands.spawn((
            DespawnOnExit(MenuState::Main),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Text::new("Bevy Game Menu UI"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Play,
                        children![
                            (ImageNode::new(right_icon), button_icon_node.clone()),
                            (
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Settings,
                        children![
                            (ImageNode::new(wrench_icon), button_icon_node.clone()),
                            (
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (ImageNode::new(exit_icon), button_icon_node),
                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                        ]
                    ),
                ]
            )],
        ));
    }

    fn settings_menu_setup(mut commands: Commands) {
        let button_node = Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        commands.spawn((
            DespawnOnExit(MenuState::Settings),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                Children::spawn(SpawnIter(
                    [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ]
                    .into_iter()
                    .map(move |(action, text)| {
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                            children![(Text::new(text), button_text_style.clone())],
                        )
                    })
                ))
            )],
        ));
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameStatus>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        game_state.set(GameStatus::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => {
                        menu_state.set(MenuState::SettingsDisplay);
                    }
                    MenuButtonAction::SettingsSound => {
                        menu_state.set(MenuState::SettingsSound);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
}
