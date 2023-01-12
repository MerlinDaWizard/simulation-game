use bevy::{prelude::*};
use bevy_mod_picking::prelude::*;
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
    for (i, comp) in Components::iter().enumerate() {
        let size = comp.get_size();
        dbg!(&comp);
        dbg!(current_down);
        current_down += size.y * SCALE * 0.5;
        let texture: Handle<Image> = ass.load(comp.get_path());
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 { x: 400.0, y: current_down, z: 11.0},
                    scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            GridLock,
            crate::game::GameRoot,
            Draggable::new(),
            DragTypeReturn::new(),
            DragOpacity(0.75),
            Size(comp.get_size()),
            ComponentLink(comp),
        ));
        current_down += size.y * 0.5 * SCALE + 5.0;
    }
}

#[derive(Component)]
pub struct TrayComponent;

#[derive(Component)]
pub struct ComponentLink(Components);

#[derive(Component)]
pub struct GridLock;

