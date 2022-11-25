//! Complex example showcasing all the features together.
//!
//! Shows how our states, fixed timestep, and custom run conditions, can all be used together!
//!
//! Also shows how run conditions could be helpful for Bevy UI button handling!
//!
//! This example has a main menu with two buttons: exit the app and enter the game.
//!
//! How to "play the game": hold spacebar to spawn colorful squares, release spacebar to make them spin! <3

use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use iyes_loopless::prelude::*;

use bevy::app::AppExit;
use bevy::window::close_on_esc;

use std::time::Duration;

use rand::prelude::*;

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
        .add_enter_system(GameState::MainMenu, setup_menu)
        // menu cleanup (state exit) systems
        .add_exit_system(GameState::MainMenu, despawn_with::<MainMenu>)
        .add_exit_system(GameState::LevelsMenu, despawn_with::<LevelsMenu>)
        // game cleanup (state exit) systems
        .add_exit_system(GameState::InGame, despawn_with::<MySprite>)
        // menu stuff
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(close_on_esc)
                .with_system(butt_interact_visual)
                // our menu button handlers
                .with_system(butt_exit.run_if(on_butt_interact::<ExitButt>))
                .with_system(butt_game.run_if(on_butt_interact::<EnterButt>))
                .into()
        )
        // in-game stuff
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(back_to_menu_on_esc)
                .with_system(clear_on_del)
                .with_system(spin_sprites.run_if_not(spacebar_pressed))
                .into()
        )
        .add_fixed_timestep_system(
            "my_fixed_update", 0,
            spawn_sprite
                // only in-game!
                .run_in_state(GameState::InGame)
                // only while the spacebar is pressed
                .run_if(spacebar_pressed)
        )
        // our other various systems:
        .add_system(debug_current_state)
        // setup our camera globally (for UI) at startup and keep it alive at all times
        .add_startup_system(setup_camera)
        .run();
}

/// Marker for our in-game sprites
#[derive(Component)]
struct MySprite;

/// Marker for the main menu entity
#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct LevelsMenu;

/// Marker for the main game camera entity
#[derive(Component)]
struct GameCamera;

/// Marker for the "Exit App" button
#[derive(Component)]
struct ExitButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
struct EnterButt;

/// Marker for the Background image
#[derive(Component)]
struct Background;

/// Reset the in-game state when pressing delete
fn clear_on_del(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Delete) || kbd.just_pressed(KeyCode::Back) {
        commands.insert_resource(NextState(GameState::InGame));
    }
}

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

/// Condition system for holding the space bar
fn spacebar_pressed(kbd: Res<Input<KeyCode>>) -> bool {
    kbd.pressed(KeyCode::Space)
}

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/// Spawn a MySprite entity
fn spawn_sprite(mut commands: Commands) {
    let mut rng = thread_rng();
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(rng.gen(), rng.gen(), rng.gen(), 0.5),
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(
            rng.gen_range(-420.0..420.0),
            rng.gen_range(-420.0..420.0),
            rng.gen_range(0.0..100.0),
        ),
        ..Default::default()
    }, MySprite));
}

/// Spawn the camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}

/// Rotate all the sprites
fn spin_sprites(mut q: Query<&mut Transform, With<MySprite>>, t: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotate(Quat::from_rotation_z(1.0 * t.delta_seconds()));
    }
}

/// Change button color on interaction
fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::rgba(0.4, 0.4, 0.4,0.5));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgba(0.2, 0.2, 0.2,0.5));
                
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgba(0.0, 0.0, 0.0,0.5));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

/// Handler for the Exit Game button
fn butt_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Handler for the Enter Game button
fn butt_game(mut commands: Commands) {
    // queue state transition
    commands.insert_resource(NextState(GameState::InGame));
}

/// Construct the main menu UI
fn setup_menu(mut commands: Commands, ass: Res<AssetServer>, mut mat: ResMut<Assets<ColorMaterial>>) {
    let butt_style = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center, // Text in middle Top / down
        align_items: AlignItems::Center, // Text in middle LR
        align_self: AlignSelf::Stretch,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        
        ..Default::default()
    };

    let butt_textstyle = TextStyle {
        font: ass.load("JetBrainsMono/JetBrainsMono-ExtraBold.ttf"),
        font_size: 32.0,
        color: Color::WHITE,
    };

    let background = commands.spawn((SpriteBundle {
        texture: ass.load("background.jpg"),

        ..Default::default()
    }, MainMenu)).id();

    let menu = commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(25.0), Val::Percent(20.0)),
                margin: UiRect::all(Val::Px(2.0)),
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                
                ..Default::default()
            },
            ..Default::default()
        }, MainMenu))
        .id();

    let butt_enter = commands
        .spawn((ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        }, EnterButt))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Enter Game", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    let butt_exit = commands
        .spawn((ButtonBundle {
            background_color: BackgroundColor(Color::rgba(1.0, 0.0, 0.0, 0.75)),
            style: butt_style.clone(),
            ..Default::default()
        }, ExitButt))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Exit Game", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    commands
        .entity(menu)
        .push_children(&[butt_enter, butt_exit]);
}
