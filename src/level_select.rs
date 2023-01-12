use bevy::{prelude::*};

#[derive(Component)]
pub struct LevelsMenu;

#[derive(Component)]
struct Row;

#[derive(Component)]
pub struct LevelButton;

#[derive(Component)]
pub struct ButtonText;

#[derive(Resource)] // TODO:
pub struct CurrentLevel(pub Option<u16>);

const LEVEL_COUNT: u16 = 35;
const MAX_ROW_LENGTH: u16 = 10;
const ROW_COUNT: u16 = LEVEL_COUNT / MAX_ROW_LENGTH + (LEVEL_COUNT % MAX_ROW_LENGTH != 0) as u16;


/// Change button color on interaction
pub fn butt_interact_visual(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                *color = BackgroundColor(Color::rgba(0.2, 0.2, 0.2,1.0));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::rgba(0.4, 0.4, 0.4,1.0));
                
            }
            Interaction::None => {
                *color = BackgroundColor(Color::rgba(0.5, 0.5, 0.5,1.0));
            }
        }
    }
}

/// Condition to help with handling multiple buttons
///
/// We get which level it is working off from the text. We do this by queryi9ng the children of the buttons to get the text and then parsing it back to u16.
pub fn on_butt_interact<B: Component>(
    _commands: Commands,
    q_parent: Query<(&Children, &Interaction), (Changed<Interaction>, With<Button>, With<B>)>, // The buttons
    q_child: Query<&Text, With<ButtonText>> // Should be the text
) {
    for (children, interaction) in q_parent.iter() {
        if *interaction == Interaction::Clicked {
            let mut level_num: u16 = 0;
            for &child in children.iter() {
                let text = q_child.get(child);
                match text {
                    Err(e) => println!("{e}"),
                    Ok(bevy_text) => {
                        println!("Found text!");
                        let text = bevy_text.sections[0].value.clone();
                        level_num = text.parse().unwrap();

                    },
                }
            }

            println!("{level_num}");
        }
    }
}

//pub fn butt_levels(mut commands: Commands) {
//    commands.insert_resource(NextState(GameState::LevelsMenu));
//}

/// Sets up level select screen using flexboxes and stuff
pub fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    let button_style = Style {
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        //align_content: todo!(),
        justify_content: JustifyContent::Center,
        margin: UiRect::all(Val::Px(4.0)),
        padding: UiRect::all(Val::Px(8.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let text_style = TextStyle {
        font: ass.load("Pixelboy.ttf"),
        font_size: 32.0,
        color: Color::BLACK,
    };

    // Contains padding and button container, used to set flex direction into column, letting padding shift down instead of to the right
    let whole = commands.spawn (
        ( NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }, LevelsMenu)
    ).id();

    // Acts as padding to shift down the button container
    let padding = commands.spawn (
        NodeBundle {
            background_color: BackgroundColor(Color::NONE),
            style: Style {
                
                size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        }
        //}, LevelsMenu)
    ).id();

    // Contains all the buttons
    let container = commands.spawn(
        NodeBundle {
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
            }
            //}, LevelsMenu)
    ).id();
    
    //let mut rows: [Option<Entity>; ROW_COUNT as usize] = [None; ROW_COUNT as usize];
    let mut rows = Vec::<Entity>::with_capacity(ROW_COUNT as usize);
    for x in 0..ROW_COUNT as usize {
        rows.push(commands.spawn( 
            (NodeBundle {
                background_color: BackgroundColor(Color::NONE),
                //background_color: BackgroundColor(Color::rgba (1.0*(x as f32/(ROW_COUNT as f32-1.0)),0.0,1.0,1.0)),
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
            let string_number = format!("{:02}",(x*MAX_ROW_LENGTH as usize+y+1)); // Change this +1 if you want to start from zero
            items.push(commands.spawn(( ButtonBundle {
                style: button_style.clone(),
                ..Default::default()
            }, LevelButton))
            .with_children(|btn| {
                btn.spawn((TextBundle {
                    text: Text::from_section(string_number, text_style.clone()),
                    ..Default::default()
                }, ButtonText));
            }).id());
        }
        commands.entity(rows[x]).push_children(&items);
    }
    commands.entity(whole).push_children(&[padding, container]);
    commands.entity(container).push_children(&rows);
}