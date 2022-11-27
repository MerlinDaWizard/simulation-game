mod main_menu;
mod level_select;
mod game;

use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use iyes_loopless::prelude::*;

use bevy::app::AppExit;
use bevy::window::close_on_esc;

use std::time::Duration;


/// Our Application State
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    LevelsMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // add out states driver
        .add_loopless_state(GameState::MainMenu)
        // Add a FixedTimestep, cuz we can!
        .add_fixed_timestep(
            Duration::from_millis(125),
            // give it a label
            "my_fixed_update",
        )
        // menu setup (state enter) systems
        .add_enter_system(GameState::MainMenu, main_menu::setup_menu)
        .add_enter_system(GameState::LevelsMenu, level_select::setup)
        // menu cleanup (state exit) systems
        .add_exit_system(GameState::MainMenu, despawn_with::<main_menu::MainMenu>)
        .add_exit_system(GameState::LevelsMenu, despawn_with::<level_select::LevelsMenu>)
        // game cleanup (state exit) systems
        .add_exit_system(GameState::InGame, despawn_with::<game::MySprite>)
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
                .with_system(game::clear_on_del)
                .with_system(game::spin_sprites.run_if_not(game::spacebar_pressed))
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
        .add_fixed_timestep_system(
            "my_fixed_update", 0,
            game::spawn_sprite
                // only in-game!
                .run_in_state(GameState::InGame)
                // only while the spacebar is pressed
                .run_if(game::spacebar_pressed)
        )
        // our other various systems:
        .add_system(debug_current_state)
        // setup our camera globally (for UI) at startup and keep it alive at all times
        .add_startup_system(setup_camera)
        .run();
}

/// Marker for the main game camera entity
#[derive(Component)]
struct GameCamera;

/// Transition back to menu on pressing Escape
fn back_to_menu_on_esc(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

/// We can just access the `CurrentState`, and even use change detection!
fn debug_current_state(state: Res<CurrentState<GameState>>) {
    if state.is_changed() {
        println!("Detected state change to {:?}!", state);
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
}



