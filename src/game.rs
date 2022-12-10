use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::components;
use crate::GameState;
use crate::components::wires::ConnectionData;
use crate::components::wires::GridPos;
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
                padding: UiRect::all(Px(0.0)),
                ..Default::default()
            },
            ..Default::default()
        }, GameRoot)).id();
    
    let component_panel = commands
        .spawn( NodeBundle {
            //background_color: BackgroundColor(Color::RED),
            style: Style {
                size: Size::new(Val::Percent(30.0),Percent(100.0)),
                margin: UiRect::left(Px(8.0)),
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
            //background_color: BackgroundColor(Color::ORANGE_RED),
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

        let top_bar = commands
        .spawn( NodeBundle {
            //background_color: BackgroundColor(Color::PINK),
            style: Style {
                size: Size::new(Percent(100.0), Percent(10.0)),
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()

            },
            ..Default::default()
        }).id();

        let middle_area = commands
        .spawn( NodeBundle {
            //background_color: BackgroundColor(Color::INDIGO),
            style: Style {
                size: Size::new(Percent(100.0), Percent(70.0)),
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()

            },
            ..Default::default()
        }).id();

        let bottom_bar = commands
        .spawn( NodeBundle {
            //background_color: BackgroundColor(Color::PURPLE),
            style: Style {
                size: Size::new(Percent(100.0), Percent(20.0)),
                align_self: AlignSelf::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                ..Default::default()

            },
            ..Default::default()
        }).id();

        //let home_button = commands
        //    .spawn( ButtonBundle {
        //        
        //    })

    commands.entity(main_side).push_children(&[top_bar, middle_area, bottom_bar]);
    commands
        .entity(root_bundle)
        .push_children(&[main_side,component_panel]);

    commands.spawn(components::wires::WireBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 { x: 50.0, y: 50.0, z: 1.0 },
                scale: Vec3::ONE,
                ..Default::default()
            },
            texture: ass.load("grid.png"),
            ..Default::default()
        },
        grid_pos: GridPos(1,1),
        connections: ConnectionData {
            up: true, down: true, left: true, right: true,
        }
    });

    commands.spawn( (SpriteBundle {
        sprite: Sprite {
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 { x: 50.0, y: 50.0, z: 100.0 },
            ..Default::default()
        },
        texture: ass.load("cursor.png"),
        ..Default::default()
    }, Cursor));
    
}

#[derive(Component)]
pub struct Cursor;

pub fn get_cursor_pos(
    windows: Res<Windows>,
    mut q: Query<&mut Transform, With<Cursor>>
) {
    let window = windows.get_primary().unwrap();
    
    if let Some(position) = window.cursor_position() {
        println!("{:?}", position);
        for mut transform in q.iter_mut() {
            println!("{:?}", transform.translation);
            transform.translation.x = position.x - (window.width()/2.0); // Mouse position is from bottom left
            transform.translation.y = position.y - (window.height()/2.0); // Whereas entity position is from middle of screen.
        }
    }

}
// fn main() {
//     let mut v = vec![vec![wire{test:1,id:1},wire{test:1,id:2},wire{test:1,id:2},wire{test:1,id:3}],vec![wire{test:2,id:1},wire{test:2,id:2},wire{test:2,id:2},wire{test:2,id:3}]];
//     for mut across in 0..v.len() {
//         for mut down in 0..v[across].len() {
//             let mut item = &v[across][down];
//             println!("=-=-=-=-=--=-=-=-=-=-=-=-=-=-=");
//             println!("{}",item.test);
//             let c = match v.get(across+1) {
//                 None => {println!("Out of range, aborting"); continue;},
//                 Some(c) => c,
//             };
            
//             let c = match c.get(down) {
//                 None => {println!("Out of range, aborting2"); continue;},
//                 Some(c) => c,
//             };
//             //item.test = item.test+1;
//             //println!("{:?}",v.get(across+1).unwrap_or_else(|| {continue;}).get(down).unwrap_or_else(|| {continue;}));
//             println!("Has to the right");
            
//         }
//     }
//     println!("Hello, world!");
// }