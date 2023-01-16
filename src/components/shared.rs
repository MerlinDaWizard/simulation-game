use std::path::PathBuf;
use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use strum_macros::EnumIter;
use enum_dispatch::enum_dispatch;
use crate::game::{PlacementGrid, GRID_CELL_SIZE, GameRoot};
use super::temp::*;
use super::wires::Wire;
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

#[enum_dispatch]
#[derive(EnumIter, Debug, PartialEq, Clone, Component)]
pub enum Components {
    WirePiece(Wire),
    GateNot(GateNot),
    GateAnd(GateAnd),
    SignalCopy(SignalCopy),
    SignalPassthrough(SignalPassthrough),

}

impl Components {
    pub fn create_default(source: &Components, pos: &GridPos) -> Components {
        dbg!(source.get_grid_pos());
        match source {
            Components::WirePiece(_) => Self::WirePiece(Wire { grid_pos: pos.0, ..Default::default()}),
            Components::GateNot(_) => Self::GateNot(GateNot { grid_pos: pos.0, ..Default::default()}),
            Components::GateAnd(_) => Self::GateAnd(GateAnd { grid_pos: pos.0, ..Default::default()}),
            Components::SignalCopy(_) => Self::SignalCopy(SignalCopy { grid_pos: pos.0, ..Default::default()}),
            Components::SignalPassthrough(_) => Self::SignalPassthrough(SignalPassthrough { grid_pos: pos.0, ..Default::default()}),
        }
    }

    pub fn get_path(&self) -> PathBuf {
        let mut p = PathBuf::from("components");
        let s = match self {
            Components::WirePiece(_) => "wire_fake.png",
            Components::GateNot(_) => "gate_not.png",
            Components::GateAnd(_) => "gate_and.png",
            Components::SignalCopy(_) => "signal_copy.png",
            Components::SignalPassthrough(_) => "signal_passthrough.png",
        };
        p.push(s);
        p
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            Components::WirePiece(_) => Vec2::splat(64.0),
            Components::GateNot(_) => Vec2::splat(64.0),
            Components::GateAnd(_) => Vec2::splat(128.0),
            Components::SignalCopy(_) => Vec2::new(64.0,128.0),
            Components::SignalPassthrough(_) => Vec2::splat(64.0),
        }
    }

    pub fn get_grid_size(&self) -> UVec2 {
        match self {
            Components::WirePiece(_) => UVec2::new(1,1),
            Components::GateNot(_) => UVec2::new(1,1),
            Components::GateAnd(_) => UVec2::new(2,2),
            Components::SignalCopy(_) => UVec2::new(1,2),
            Components::SignalPassthrough(_) => UVec2::new(1,1),
        }
    }
}

#[enum_dispatch(Components)]
pub trait GridComponent {
    fn get_grid_pos(&self) -> UVec2;
}

/// Represents the size of a sprite used in some cases for gridlocking where not every sprite is exactly 1 grid size
/// 
/// I could calculate this on the fly using the image however this is easier (due to no async loading) and probs more efficient
#[derive(Component)]
pub struct Size(pub Vec2);

/// Represents a components position in the placement grid
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GridPos(pub UVec2);

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
        }, GameRoot, Components::create_default(&placement.1, &placement.0))); // TODO! Component for board components and functionality
    }
}

fn calc_grid_pos(comp: &Components, grid_bottom_left: &Vec2, pos_in_grid: &GridPos) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.0.as_vec2() * GRID_CELL_SIZE) + comp.get_size() * 0.5;
    pos
}