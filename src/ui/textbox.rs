use bevy::{prelude::*, input::{mouse::{MouseMotion, MouseButtonInput}, ButtonState}, render::camera::RenderTarget};
use crate::{GameCamera, game::Interactable};

#[derive(Component)]
pub struct BoxRoot;

#[derive(Component)]
pub struct BoxTextBox;

#[derive(Component)]
pub struct BoxLineNumbers;

#[derive(Component)]
pub struct BoxTitle;

#[derive(Component)]
pub struct BoxCloseButton;

#[derive(Component)]
pub struct BoxHideButton;

#[derive(Bundle)]
pub struct BoxRootBundle {
    #[bundle]
    pub(crate) sprite: SpriteBundle,
    pub(crate) box_root: BoxRoot,
    pub(crate) held: DragState
}

#[derive(Component, PartialEq)]
pub enum DragState {
    Held,
    Dropped,

}

pub struct ProgramBox {
    root: Entity,
    name_text: Entity,
}

impl ProgramBox {
    pub fn new<S: Into<String>, T: Component>(mut commands: Commands, ass: Res<AssetServer>, name: S, root_type: T) {
        let box_top = commands.spawn((BoxRootBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(70, 70, 70),
                    custom_size: Some(Vec2::new(200.0,35.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: 0.0, z: 2.0 },
                    ..Default::default()
                },
                //texture: todo!(),
                //visibility: todo!(),
                ..Default::default()
            },
            box_root: BoxRoot,
            held: DragState::Held,
        }, root_type)).id();
    
        let box_name = commands.spawn( Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(name, TextStyle { font: ass.load("Pixelboy.ttf"), font_size: 24.0, color: Color::WHITE })],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left },
            },
            transform: Transform {
                translation: Vec3 { x: -95.0, y: 12.0, z: 1.1 },
                ..Default::default()
            },
            ..Default::default()
        }).id();
        
        let box_exit = commands.spawn(( SpriteBundle {
            texture: ass.load("exit_button.png"),
            transform: Transform {
                translation: Vec3 { x: 83.0, y: 0.0, z: 1.2 },
                ..Default::default()
            },
            ..Default::default()
        }, BoxCloseButton)).id();

        commands.entity(box_top).push_children(&[box_name, box_exit]);
    }
}

pub fn drag_system(
    mut query: Query<(&mut Transform, &DragState), (With<BoxRoot>)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut mouse_evr: EventReader<MouseMotion>,
) {
    for ev in mouse_evr.iter() {
        for mut obj in query.iter_mut() {
            if *obj.1 == DragState::Held {
                obj.0.translation.x += ev.delta.x;
                obj.0.translation.y += -ev.delta.y;
            }
        }
    }
}

pub fn click_system(
    mut query: Query<(Entity, &GlobalTransform, &Sprite, Option<&mut DragState>), With<Interactable>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    wnds: Res<Windows>,
    mut button_evr: EventReader<MouseButtonInput>,
) {
    for ev in button_evr.iter() {
        if ev.button == MouseButton::Left {
            // Thanks to bevy cookbook for the code to convert window position to sprite position :D
            // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
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
                // Loop here first,
                for (entity, global_transform, sprite, draggable) in query.iter() {
                    let pos = global_transform.translation();
                    
                }
                match ev.state {
                    ButtonState::Pressed => {
                        
                    },
                    ButtonState::Released => {
                        
                    },
                }
            } else {
                // Should drop if mouse cursor released off screen
            }
        }
    }
}