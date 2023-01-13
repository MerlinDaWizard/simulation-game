use std::path::PathBuf;
use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use strum_macros::EnumIter;

use crate::game::{PlacementGrid, GRID_CELL_SIZE, GameRoot};

pub struct ComponentSetupPlugin;

impl Plugin for ComponentSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlaceComponentEvent>()
            //.add_enter_system(crate::GameState::InGame, enter_system)
            .add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::InGame)
                .with_system(placement_event)
                // .with_system(ui_example_system)
                .into()
        );
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum Components {
    WirePiece,
    GateNot,
    GateAnd,
    SignalCopy,
    SignalPassthrough,

}

impl Components {
    pub fn get_path(&self) -> PathBuf {
        let mut p = PathBuf::from("components");
        let s = match self {
            Components::WirePiece => "wire_fake.png",
            Components::GateNot => "gate_not.png",
            Components::GateAnd => "gate_and.png",
            Components::SignalCopy => "signal_copy.png",
            Components::SignalPassthrough => "signal_passthrough.png",
        };
        p.push(s);
        p
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            Components::WirePiece => Vec2::splat(64.0),
            Components::GateNot => Vec2::splat(64.0),
            Components::GateAnd => Vec2::splat(128.0),
            Components::SignalCopy => Vec2::new(64.0,128.0),
            Components::SignalPassthrough => Vec2::splat(64.0),
        }
    }
}

/// Represents the size of a sprite used in some cases for gridlocking where not every sprite is exactly 1 grid size
/// 
/// I could calculate this on the fly using the image however this is easier (due to no async loading) and probs more efficient
#[derive(Component)]
pub struct Size(pub Vec2);

/// Represents a components position in the placement grid
#[derive(Debug, Component)]
pub struct GridPos(pub u8,pub u8);

impl GridPos {
    fn create_vec2(&self) -> Vec2 {
        Vec2 { x: self.0 as f32, y: self.1 as f32 }
    }
}
pub struct PlaceComponentEvent(pub GridPos, pub Components);

fn placement_event(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut place_ev: EventReader<PlaceComponentEvent>,
    placement_grid: Query<(&Sprite, &Transform, &Size, With<PlacementGrid>)>,
) {
    let grid = placement_grid.single();
    let size = grid.2;
    let grid_bottom_left = grid.1.translation.truncate() - (size.0 * 0.5);
    for placement in place_ev.iter() {
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform {
                translation: calc_grid_pos(&placement.1, &grid_bottom_left, &placement.0).extend(11.0),
                scale: Vec3::splat(2.0),
                ..Default::default()
                
            },
            texture: ass.load(placement.1.get_path()),
            ..Default::default()
        }, GameRoot)); // TODO! Component for board components and functionality
    }
}

fn calc_grid_pos(comp: &Components, grid_bottom_left: &Vec2, pos_in_grid: &GridPos) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.create_vec2() * GRID_CELL_SIZE) + comp.get_size() * 0.5;
    pos
}