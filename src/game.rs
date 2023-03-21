
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::PrimaryWindow;
use crate::components::grid::Grid;
use crate::components::shared::Size;
use crate::level_select::CurrentLevel;
use crate::{ui, MainTextureAtlas, GameCamera};

pub const GRID_CELL_SIZE: f32 = 32.0;
pub const GRID_CELL_AMOUNT_WIDTH: u8 = 7;
pub const GRID_CELL_AMOUNT_HEIGHT: u8 = 7;

/// Root component for this screen
#[derive(Component)]
pub struct GameRoot;

#[derive(Component)]
pub struct PlacementGrid;

/// Sets up screen using flex boxies and loads components etc.
pub fn setup_screen(
    mut commands: Commands,
    ass: Res<AssetServer>,
    _level: Res<CurrentLevel>,

    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>) { // At the moment `CurrentLevel` actually refers to the level to load
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 { x: -60.0, y: 35.0, z: 10.0 },
            //scale: Vec3::splat(2.0),
            ..Default::default()
        },
        texture: ass.load("grid.png"),
        ..Default::default()
    }, GameRoot, Name::new("Placement Grid"), PlacementGrid, Size(Vec2::new(GRID_CELL_AMOUNT_WIDTH as f32,GRID_CELL_AMOUNT_HEIGHT as f32)*GRID_CELL_SIZE)));

    let cursor = commands.spawn( (SpriteBundle {
        sprite: Sprite {
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 { x: 50.0, y: 50.0, z: 99.0 },
            ..Default::default()
        },
        texture: ass.load("cursor.png"),
        ..Default::default()
    }, Cursor, GameRoot)).id();

    let cursor_inside = commands.spawn( (SpriteBundle {
        sprite: Sprite {
            color: Color::GREEN,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3 { x: 0.0, y: 0.0, z: 100.1 },
            scale: Vec3::new(0.5,0.5,1.0),
            ..Default::default()
        },
        texture: ass.load("cursor.png"),
        ..Default::default()
    }, CursorInside, GameRoot)).id();
    commands.entity(cursor).add_child(cursor_inside);

    ui::textbox::ProgramBox::new(&mut commands, &ass, &atlases, &main_atlas, "A1", GameRoot);
    ui::textbox::ProgramBox::new(&mut commands, &ass, &atlases, &main_atlas, "A2", GameRoot);

}

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct CursorInside;

/// Unit component to mark an entity as interactable for the click_system
#[derive(Component)]
pub struct Interactable;

pub fn get_cursor_pos(
    wnds: Query<&Window, With<PrimaryWindow>>,
    kbd: Res<Input<KeyCode>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mut main_query: Query<&mut Transform, (With<Cursor>, Without<CursorInside>)>,
    mut inside_cursor: Query<&mut Transform, (With<CursorInside>, Without<Cursor>)>
) {
    
    if kbd.pressed(KeyCode::Space) {
            // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            // get the size of the window
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();

            eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
            for mut transform in main_query.iter_mut() {
                //println!("{:?}", transform.translation);
                transform.translation.x = world_pos.x; // Mouse position is from bottom left
                transform.translation.y = world_pos.y; // Whereas entity position is from middle of screen.
                transform.rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.05));
            }

            for mut transform in inside_cursor.iter_mut() {
                transform.rotate_local(Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -0.05));
            }
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
