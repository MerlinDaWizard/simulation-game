use bevy::{prelude::*};
use bevy_mod_picking::prelude::*;
use iyes_loopless::prelude::*;
use strum::IntoEnumIterator;
use crate::{components::shared::*};
use crate::ui::shared::*;

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

fn enter_system(mut commands: Commands, ass: Res<AssetServer>) {
    for (i, comp) in Components::iter().enumerate() {
        let texture: Handle<Image> = ass.load(comp.get_path());
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3 { x: 400.0, y: (-200.0+100.0*i as f32), z: 11.0 },
                    scale: Vec3::splat(2.0),
                    ..Default::default()
                },
                texture,
                ..Default::default()
            },
            ComponentLink(comp),
            GridLock,
            crate::game::GameRoot,
            Draggable::new(),
            DragTypeReturn::new(),
            DragOpacity(0.5),
        ));

    }
}

#[derive(Component)]
pub struct TrayComponent;

#[derive(Component)]
pub struct ComponentLink(Components);

#[derive(Component)]
pub struct GridLock;

