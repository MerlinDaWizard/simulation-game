use bevy::{prelude::*, app::AppExit};
use iyes_loopless::prelude::*;

use crate::GameState;

/// Marker for the main menu entity
#[derive(Component)]
pub struct MainMenu;

/// Marker for the "Exit App" button
#[derive(Component)]
pub struct ExitButt;

#[derive(Component)]
pub struct LevelsButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
pub struct EnterButt;

/// Marker for the "Enter Game" button
#[derive(Component)]
pub struct TestButt;

/// Marker for the Background image
#[derive(Component)]
pub struct Background;

/// Change button color on interaction
pub fn butt_interact_visual(
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
pub fn on_butt_interact<B: Component>(
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
pub fn butt_exit(mut ev: EventWriter<AppExit>) {
    ev.send(AppExit);
}

/// Handler for the Enter Game button
pub fn butt_game(mut commands: Commands) {
    // queue state transition
    commands.insert_resource(NextState(GameState::InGame));
}

pub fn butt_levels(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::LevelsMenu));
}


pub fn butt_test() {
    //crate::components::wires::t
}

/// Construct the main menu UI
pub fn setup_menu(mut commands: Commands, ass: Res<AssetServer>) {
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

    let _background = commands.spawn((SpriteBundle {
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

    let butt_levels = commands
        .spawn((ButtonBundle {
            style: butt_style.clone(),
            ..Default::default()
        }, LevelsButt))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Level Select", butt_textstyle.clone()),
                ..Default::default()
            });
        })
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

    // button for testing small snippets
    // let butt_test = commands
    //     .spawn((ButtonBundle {
    //         background_color: BackgroundColor(Color::rgba(1.0, 0.0, 0.0, 0.75)),
    //         style: butt_style,
    //         ..Default::default()
    //     }, TestButt))
    //     .with_children(|btn| {
    //         btn.spawn(TextBundle {
    //             text: Text::from_section("Test!", butt_textstyle.clone()),
    //             ..Default::default()
    //         });
    //     })
    //     .id();

    commands
        .entity(menu)
        //.push_children(&[butt_enter, butt_levels, butt_exit, butt_test]);
        .push_children(&[butt_enter, butt_levels, butt_exit]);
}