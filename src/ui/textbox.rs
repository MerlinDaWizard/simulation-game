use bevy::prelude::*;

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

pub struct ProgramBox {
    root: Entity,
    name_text: Entity,
}

impl ProgramBox {
    pub fn new<S: Into<String>, T: Component>(mut commands: Commands, ass: Res<AssetServer>, name: S, root_type: T) {
        let box_top = commands.spawn((BoxRootBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
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
        }, root_type)).id();
    
        let box_name = commands.spawn( Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(name, TextStyle { font: ass.load("Pixelboy.ttf"), font_size: 24.0, color: Color::GRAY })],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left },
            },
            transform: Transform {
                translation: Vec3 { x: 10.0, y: 10.0, z: 1.1 },
                ..Default::default()
            },
            ..Default::default()
        }).id();
        
        commands.entity(box_top).add_child(box_name);
    }
}
