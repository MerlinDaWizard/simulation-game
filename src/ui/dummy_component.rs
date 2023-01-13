use bevy::{prelude::*};
use iyes_loopless::prelude::*;
use strum::IntoEnumIterator;
use crate::{components::shared::*};
use crate::ui::shared::*;
use crate::components::shared::Size;
pub struct ComponentTrayPlugin;

impl Plugin for ComponentTrayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(crate::GameState::InGame, enter_system)
            .add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::InGame)
                // .with_system(ui_example_system)
                .into()
        );
    }
}

pub const SCALE: f32 = 2.0;
fn enter_system(mut commands: Commands, ass: Res<AssetServer>) {
    let mut current_down = -250.0;
    for comp in Components::iter() {
        let size = comp.get_size();
        dbg!(&comp);
        dbg!(current_down);
        current_down += size.y * 0.5;
        let texture: Handle<Image> = ass.load(comp.get_path());
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 { x: 400.0, y: current_down, z: 20.0},
                    scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            GridLock::new(),
            crate::game::GameRoot,
            Draggable::new(),
            DragTypeReturn::new(),
            DragOpacity(0.75),
            Size(comp.get_size()),
            ComponentLink(comp),
        ));
        current_down += size.y * 0.5 + 5.0;
    }
}

#[derive(Component)]
pub struct TrayComponent;

#[derive(Component)]
pub struct ComponentLink(pub Components);

#[derive(Component)]
pub struct GridLock{
    pub grab_part: Vec2
}

impl GridLock {
    pub fn new() -> Self {
        GridLock { grab_part: Vec2::ZERO }
    }
}

