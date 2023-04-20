#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
extern crate glob;

mod components;
mod config;
mod game;
mod level_select;
mod main_menu;
mod settings;
mod sim;
mod ui;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};
//use bevy_mod_picking::{DefaultPickingPlugins, DebugEventsPickingPlugin, PickingCameraBundle};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::window::{close_on_esc, PresentMode};
use bevy_heterogeneous_texture_atlas_loader::*;
use main_menu::MainMenuPlugin;
use merlin_pick_backend::MerlinSpriteBackend;
use sim::run::SimRunPlugin;
use sim::save_load::SimLoadPlugin;

/// Main application state
/// Typically refers to the type of screen and rough file type.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    MainMenu2,
    Settings,
    LevelsMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Simulation game!".to_string(),
                        resolution: (1920., 1080.).into(),
                        present_mode: PresentMode::AutoVsync,
                        mode: bevy::window::WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Resources
        .insert_resource(Msaa::Sample2)
        .insert_resource(ClearColor(Color::rgb_u8(30, 32, 48)))
        .insert_resource(level_select::CurrentLevel(Some(1))) // TODO: Change to none + working level select
        // Plugins (foreign)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::rgb(0.1, 0.1, 0.1),
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(MerlinSpriteBackend)
        .add_plugin(EguiPlugin)
        //.add_plugin(bevy_framepace::FramepacePlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TextureAtlasLoaderPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(SimRunPlugin)
        .add_plugin(SimLoadPlugin)
        // add out states driver
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu2),
        )
        .add_collection_to_loading_state::<_, MainTextureAtlas>(GameState::Loading)
        // Own plugins
        .add_plugin(crate::ui::textbox::TextboxPlugin)
        .add_plugin(crate::ui::dummy_component::ComponentTrayPlugin)
        .add_plugin(crate::components::placement::ComponentSetupPlugin)
        .add_plugin(crate::ui::egui::main::LeftPanelPlugin)
        .add_plugin(crate::ui::egui::theming::EguiThemingPlugin)
        .add_plugin(crate::config::SettingsPlugin)
        .add_plugin(crate::settings::SettingsMenuPlugin)
        .add_plugin(crate::sim::interactions::GridComponentInteractionPlugin)
        // menu setup (state enter) systems
        .add_system(level_select::setup.in_schedule(OnEnter(GameState::LevelsMenu)))
        .add_system(game::setup_screen.in_schedule(OnEnter(GameState::InGame)))
        // menu cleanup (state exit) systems
        .add_system(
            despawn_with::<level_select::LevelsMenu>.in_schedule(OnExit(GameState::LevelsMenu)),
        )
        // game cleanup (state exit) systems
        .add_system(despawn_with::<game::GameRoot>.in_schedule(OnExit(GameState::InGame)))
        // menu stuff
        // in-game stuff
        .add_systems((back_to_menu_on_esc,).distributive_run_if(in_state(GameState::InGame)))
        // Levels menu
        .add_systems(
            (
                level_select::butt_interact_visual,
                level_select::on_butt_interact::<level_select::LevelButton>,
                back_to_menu_on_esc,
            )
                .distributive_run_if(in_state(GameState::LevelsMenu)),
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
        commands.insert_resource(NextState(Some(GameState::MainMenu2)));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
fn debug_current_state(state: Res<State<GameState>>) {
    if state.is_changed() {
        info!("Detected state change to {state:?}!");
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
    commands.spawn((PixelCameraBundle::from_resolution(640, 360), GameCamera));
}

#[derive(AssetCollection, Resource, Deref, DerefMut)]
pub struct MainTextureAtlas {
    #[asset(path = "sprite_map.ron")]
    handle: Handle<TextureAtlas>,
}
