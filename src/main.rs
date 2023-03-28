#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

mod main_menu;
mod level_select;
mod game;
mod components;
mod ui;
mod config;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{PixelCameraPlugin, PixelBorderPlugin, PixelCameraBundle};
//use bevy_mod_picking::{DefaultPickingPlugins, DebugEventsPickingPlugin, PickingCameraBundle};
use bevy::window::{close_on_esc, PresentMode, WindowResolution, WindowMode};
use bevy::diagnostic::{LogDiagnosticsPlugin};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy_heterogeneous_texture_atlas_loader::*;

/// Our Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    LevelsMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simulation game!".to_string(),
                resolution: WindowResolution::new(1920.,1080.),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }
            ..default()),
            exit_condition: bevy::window::ExitCondition::OnAllClosed,
            close_when_requested: true,
        }))
        // Resources
        .insert_resource(Msaa::Sample1)
        .insert_resource(ClearColor(Color::rgb_u8(30, 32, 48)))
        .insert_resource(level_select::CurrentLevel(None))
        // Plugins (foreign)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EguiPlugin)
        //.add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TextureAtlasLoaderPlugin)
        // add out states driver
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::MainMenu)
            .with_collection::<MainTextureAtlas>()
        )
        // Own plugins
        .add_plugin(crate::ui::textbox::TextboxPlugin)
        .add_plugin(crate::ui::dummy_component::ComponentTrayPlugin)
        .add_plugin(crate::components::shared::ComponentSetupPlugin)
        .add_plugin(crate::ui::egui::main::LeftPanelPlugin)
        .add_plugin(crate::ui::egui::theming::EguiThemingPlugin)
        .add_plugin(crate::config::SettingsPlugin)
        // menu setup (state enter) systems
        .add_system_to_schedule(GameState::MainMenu, main_menu::setup_menu)
        .add_enter_system(GameState::LevelsMenu, level_select::setup)
        .add_enter_system(GameState::InGame, game::setup_screen)
        // menu cleanup (state exit) systems
        .add_exit_system(GameState::MainMenu, despawn_with::<main_menu::MainMenu>)
        .add_exit_system(GameState::LevelsMenu, despawn_with::<level_select::LevelsMenu>)
        // game cleanup (state exit) systems
        .add_exit_system(GameState::InGame, despawn_with::<game::GameRoot>)
        // menu stuff
        .add_system_set(
            Condition::new()
                .run_in_state(GameState::MainMenu)
                .with_system(close_on_esc)
                .with_system(main_menu::butt_interact_visual)
                // our menu button handlers
                .with_system(main_menu::butt_exit.run_if(main_menu::on_butt_interact::<main_menu::ExitButt>))
                .with_system(main_menu::butt_game.run_if(main_menu::on_butt_interact::<main_menu::EnterButt>))
                .with_system(main_menu::butt_levels.run_if(main_menu::on_butt_interact::<main_menu::LevelsButt>))
                .into()
        )
        // in-game stuff
        .add_system_set(
            Condition::new()
                .run_in_state(GameState::InGame)
                .with_system(back_to_menu_on_esc)
                .with_system(game::get_cursor_pos)
                .into()
        )
        .add_system_set(
            Condition::new()
                .run_in_state(GameState::LevelsMenu)
                .with_system(level_select::butt_interact_visual)
                .with_system(level_select::on_butt_interact::<level_select::LevelButton>)
                //.with_system(level_select::butt_levels.run_if(level_select::on_butt_interact::<level_select::LevelButton>))
                .with_system(back_to_menu_on_esc)
                .into()
        )
        // our other various systems:
        .add_system(debug_current_state)
        // setup our camera globally (for UI) at startup and keep it alive at all times
        .add_startup_system(setup)
        .run();
}

/// Marker for the main game camera entity
#[derive(Component)]
pub struct GameCamera;

/// Transition back to menu on pressing Escape
fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(GameState::MainMenu)));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
fn debug_current_state(state: Res<State<GameState>>) {
    if state.is_changed() {
        println!("Detected state change to {state:?}!");
    }
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Spawn the camera & assets
fn setup(mut commands: Commands) {
    //commands.spawn((Camera2dBundle::default(), GameCamera));
    //let texture_atlas: Handle<TextureAtlas> = ass.load("sprite_map.ron");
    commands.spawn((PixelCameraBundle::from_resolution(640, 360), GameCamera));
    //commands.insert_resource(MainTextureAtlas{handle:});
    //commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default(), GameCamera));
}

#[derive(AssetCollection, Resource, Deref, DerefMut)]
pub struct MainTextureAtlas{
    #[asset(path = "sprite_map.ron")]
    handle: Handle<TextureAtlas>
}