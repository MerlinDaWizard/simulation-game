use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;

use crate::GameState;
use crate::level_select::CurrentLevel;
use Val::*;

/// Root component for this screen
#[derive(Component)]
pub struct GameRoot;

/// Sets up screen using flex boxies and loads components etc.
pub fn setup_screen(mut commands: Commands, ass: Res<AssetServer>, level: Res<CurrentLevel>) { // At the moment `CurrentLevel` actually refers to the level to load
    let root_bundle = commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Percent(100.0),Percent(100.0)),
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }, GameRoot)).id();
    
    let component_panel = commands
        .spawn( NodeBundle {
            background_color: BackgroundColor(Color::RED),
            style: Style {
                size: Size::new(Percent(29.0),Percent(100.0)),
                margin: UiRect::left(Percent(1.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        }).id();

    let main_side = commands
        .spawn( NodeBundle {
            background_color: BackgroundColor(Color::ORANGE_RED),
            style: Style {
                size: Size::new(Percent(70.0), Percent(100.0)),
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                ..Default::default()

            },
            ..Default::default()
        }).id();

    commands
        .entity(root_bundle)
        .push_children(&[main_side,component_panel]);
}