use bevy::{prelude::*, input::{mouse::{MouseMotion, MouseButtonInput}, ButtonState}, render::camera::RenderTarget};
use crate::{GameCamera, game::Interactable};
use bevy_mod_picking::prelude::{backends::sprite::SpriteBackend, *};
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
}

#[derive(Component)]
pub struct Draggable {
    pub offset: Vec2,
}

impl Draggable {
    pub fn new() -> Draggable {
        Draggable {
            offset: Vec2::ZERO,
        }
    }
}
pub struct ProgramBox {
    root: Entity,
    name_text: Entity,
}

impl ProgramBox {
    pub fn new<S: Into<String>, T: Component>(mut commands: Commands, ass: Res<AssetServer>, name: S, root_type: T) -> Entity {
        let box_top = commands.spawn((BoxRootBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb_u8(70, 70, 70),
                    custom_size: Some(Vec2::new(200.0,35.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: 0.0, z: 200.0 },
                    ..Default::default()
                },
                //texture: todo!(),
                //visibility: todo!(),
                ..Default::default()
            },
            box_root: BoxRoot,
        }, root_type, Draggable::new())).id();
    
        let box_name = commands.spawn( Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(name, TextStyle { font: ass.load("Pixelboy.ttf"), font_size: 24.0, color: Color::WHITE })],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left },
            },
            transform: Transform {
                translation: Vec3 { x: -95.0, y: 12.0, z: 200.1 },
                ..Default::default()
            },
            ..Default::default()
        }).id();
        
        let box_exit = commands.spawn(( SpriteBundle {
            texture: ass.load("exit_button.png"),
            transform: Transform {
                translation: Vec3 { x: 83.0, y: 0.0, z: 200.2 },
                ..Default::default()
            },
            ..Default::default()
        }, BoxCloseButton)).id();

        commands.entity(box_top).push_children(&[box_name, box_exit]);
        return box_top;
    }
}

pub fn drag_v2(
    mut commands: Commands,
    mut drag_start_events: EventReader<PointerDragStart>,
    mut drag_events: EventReader<PointerDrag>,
    pointers: Res<PointerMap>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    locations: Query<&PointerLocation>,
    mut boxes: Query<((Entity, &mut Draggable), &mut Transform)>,

) {
    for start in drag_start_events.iter() {
        
        let ((_, mut draggable), transform) = match boxes.get_mut(start.target()) {
            Ok(b) => b,
            Err(_)=> {
                continue;
            }
        };

        let pointer_entity = pointers.get_entity(start.pointer_id()).unwrap();
        let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
        let pointer_position = pointer_location.position;
        let target = pointer_location
            .target
            .get_render_target_info(&windows, &images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
        
        draggable.offset = transform.translation.truncate() - (pointer_position - (target_size / 2.0));
        
    }

    for dragging in drag_events.iter() {
        let pointer_entity = pointers.get_entity(dragging.pointer_id()).unwrap();
        let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
        let pointer_position = pointer_location.position;
        let target = pointer_location
            .target
            .get_render_target_info(&windows, &images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
        //dbg!(&boxes);
        //dbg!(&dragging.target());
        let ((_, mut draggable), mut box_transform) = match boxes.get_mut(dragging.target()) {
            Ok(e) => e,
            Err(e) => {
                continue;
            }
        };

        let z = box_transform.translation.z;
        box_transform.translation = (pointer_position - (target_size / 2.0) + draggable.offset).extend(z);
        println!("==============");
    }
}