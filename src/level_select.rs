use bevy::{prelude::*, log::Level};
use iyes_loopless::prelude::*;

#[derive(Component)]
pub struct LevelsMenu;

#[derive(Component)]
struct Row;

#[derive(Component)]
struct LevelButton;

const BUTTON_COLOUR: Color = Color::rgba(0.0, 0.0, 0.0,0.5);
const LEVEL_COUNT: u16 = 35;
const MAX_ROW_LENGTH: u16 = 10;
const ROW_COUNT: u16 = LEVEL_COUNT / MAX_ROW_LENGTH + (LEVEL_COUNT % MAX_ROW_LENGTH != 0) as u16;

pub fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    let button_style = Style {
        align_items: AlignItems::Center,
        //align_content: todo!(),
        justify_content: JustifyContent::Center,
        margin: UiRect::all(Val::Px(4.0)),
        padding: UiRect::all(Val::Px(8.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let text_style = TextStyle {
        font: ass.load("JetBrainsMono/JetBrainsMono-ExtraBold.ttf"),
        font_size: 32.0,
        color: Color::BLACK,
    };

    let outer_node = commands.spawn(
        (NodeBundle {
                background_color: BackgroundColor(Color::CYAN),
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    //align_items: AlignItems::Stretch,
                    margin: UiRect::all(Val::Auto),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    //padding: todo!(),
                    ..Default::default()
                },
                ..Default::default()
            }, LevelsMenu)
    ).id();
    
    //let mut rows: [Option<Entity>; ROW_COUNT as usize] = [None; ROW_COUNT as usize];
    let mut rows = Vec::<Entity>::with_capacity(ROW_COUNT as usize);
    for x in 0..ROW_COUNT as usize {
        rows.push(commands.spawn(
            (NodeBundle {
                background_color: BackgroundColor(Color::rgba (1.0*(x as f32/(ROW_COUNT as f32-1.0)),0.0,1.0,1.0)),
                style: Style {
                    //flex_direction: FlexDirection::Row,
                    //align_items: AlignItems::FlexStart,
                    //justify_content: JustifyContent::FlexStart,
                    margin: UiRect::all(Val::Px(2.0)),
                    ..Default::default()
                },
                ..Default::default()
            }, Row)
        ).id());

        let row_length = std::cmp::min(LEVEL_COUNT as usize - (x * MAX_ROW_LENGTH as usize), 10);
        let mut items = Vec::<Entity>::with_capacity(10);
        for y in 0..row_length {
            let string_number = format!("{:02}",(x*MAX_ROW_LENGTH as usize+y+1));
            items.push(commands.spawn(( ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            }, LevelButton))
            .with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section(string_number, text_style.clone()),
                    ..Default::default()
                });
            }).id());
        }

        commands.entity(rows[x]).push_children(&items);
    }    
    commands.entity(outer_node).push_children(&rows);
}