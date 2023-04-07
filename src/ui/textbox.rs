use bevy::{prelude::*, ui::FocusPolicy};

use bevy_mod_picking::prelude::{*};
use iyes_loopless::prelude::*;
use crate::{ui::shared::*, MainTextureAtlas};

pub struct TextboxPlugin;

impl Plugin for TextboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::InGame)
                .with_system(drag_v2)
                .with_system(CloseBox::handle_events)
                .into()
        );
    }
}
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
    pub(crate) sprite: SpriteSheetBundle,
    pub(crate) box_root: BoxRoot,
}

pub struct ProgramBox {
    root: Entity,
    name_text: Entity,
    exit_button: Entity,
}

impl ProgramBox {
    pub fn new<S: Into<String>, T: Component>(commands: &mut Commands, ass: &Res<AssetServer>, atlases: &Res<Assets<TextureAtlas>>, main_atlas: &Res<MainTextureAtlas>, name: S, root_type: T) -> Self {
        let name: String = name.into();
        let texture_atlas = atlases.get(&main_atlas.handle).unwrap();
        let box_top = commands.spawn((BoxRootBundle {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(texture_atlas.get_texture_index(&Handle::weak("box_root".into())).unwrap()),
                transform: Transform {
                    translation: Vec3 { x: 0.0, y: 0.0, z: 200.0 },
                    ..Default::default()
                },
                texture_atlas: main_atlas.handle.clone(),
                ..Default::default()
            },
            box_root: BoxRoot,
        },
        root_type,
        Draggable::new(),
        Name::new(format!("Box - {}", &name))
    )).id();
    
        let box_name = commands.spawn( (Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(name, TextStyle { font: ass.load("Pixelboy.ttf"), font_size: 24.0, color: Color::WHITE })],
                alignment: TextAlignment { vertical: VerticalAlign::Top, horizontal: HorizontalAlign::Left },
            },
            transform: Transform {
                translation: Vec3 { x: -95.0, y: 12.0, z: 200.1 },
                ..Default::default()
            },
            ..Default::default()
        }, BoxTitle)).id();
        
        let box_exit = commands.spawn(( SpriteBundle {
            texture: ass.load("exit_button.png"),
            transform: Transform {
                translation: Vec3 { x: 83.0, y: 0.0, z: 200.2 },
                ..Default::default()
            },
            ..Default::default()
        }, BoxCloseButton, FocusPolicy::Block, PickableBundle::default()))
            .forward_events::<PointerClick, CloseBox>()
            .id();

        commands.entity(box_top).push_children(&[box_name, box_exit]);
        ProgramBox { root: box_top, name_text: box_name, exit_button: box_exit}
    }
}

// Event for closing a box
struct CloseBox(Entity);

impl ForwardedEvent<PointerClick> for CloseBox {
    fn from_data(event_data: &PointerEventData<PointerClick>) -> CloseBox {
        CloseBox(event_data.target())
    }
}

impl CloseBox {
    fn handle_events(
        _commands: Commands,
        mut close: EventReader<CloseBox>,
        mut q_boxes: Query<(&Children, Entity, &mut Visibility), With<BoxRoot>>,
        q_titles: Query<(&Parent, &Text), With<BoxTitle>>,
    ) {
        for event in close.iter() {
            for (children, box_root, mut visibility) in q_boxes.iter_mut() {
                if !children.contains(&event.0) {
                    continue;
                }
                visibility.is_visible = false;

                for (bot_root_list, text) in q_titles.iter() {
                    if bot_root_list.get() != box_root {
                        continue;
                    }

                    debug!("Box {} pressed", text.sections[0].value);
                }
            }
            //commands.entity(event.0)
        }
    }
}
