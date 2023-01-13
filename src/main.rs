#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

mod main_menu;
mod level_select;
mod game;
mod components;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;
//use bevy_mod_picking::{DefaultPickingPlugins, DebugEventsPickingPlugin, PickingCameraBundle};
use iyes_loopless::prelude::*;
use bevy::window::close_on_esc;
use bevy::diagnostic::{LogDiagnosticsPlugin};
use std::time::Duration;


/// Our Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    LevelsMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        // add out states driver
        .add_loopless_state(GameState::MainMenu)
        .add_plugin(crate::ui::textbox::TextboxPlugin)
        .add_plugin(crate::ui::dummy_component::ComponentTrayPlugin)
        .add_plugin(crate::components::shared::ComponentSetupPlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(bevy_framepace::FramepacePlugin)
        // Add a FixedTimestep, cuz we can!
        .add_fixed_timestep(
            Duration::from_millis(125),
            // give it a label
            "my_fixed_update",
        )
        .insert_resource(level_select::CurrentLevel(None))
        // menu setup (state enter) systems
        .add_enter_system(GameState::MainMenu, main_menu::setup_menu)
        .add_enter_system(GameState::LevelsMenu, level_select::setup)
        .add_enter_system(GameState::InGame, game::setup_screen)
        // menu cleanup (state exit) systems
        .add_exit_system(GameState::MainMenu, despawn_with::<main_menu::MainMenu>)
        .add_exit_system(GameState::LevelsMenu, despawn_with::<level_select::LevelsMenu>)
        // game cleanup (state exit) systems
        .add_exit_system(GameState::InGame, despawn_with::<game::GameRoot>)
        // menu stuff
        .add_system_set(
            ConditionSet::new()
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
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(back_to_menu_on_esc)
                .with_system(game::get_cursor_pos)
                .into()
        )
        .add_system_set(
            ConditionSet::new()
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
        .add_startup_system(setup_camera)
        .run();
}

/// Marker for the main game camera entity
#[derive(Component)]
pub struct GameCamera;

/// Transition back to menu on pressing Escape
fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
fn debug_current_state(state: Res<CurrentState<GameState>>) {
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

/// Spawn the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
    //commands.spawn((Camera2dBundle::default(), PickingCameraBundle::default(), GameCamera));
}
