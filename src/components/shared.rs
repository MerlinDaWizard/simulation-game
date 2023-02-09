use bevy::prelude::*;
use bevy::sprite::Anchor;
use iyes_loopless::prelude::{ConditionSet, AppLooplessStateExt};
use strum_macros::EnumIter;
use enum_dispatch::enum_dispatch;
use crate::MainTextureAtlas;
use crate::game::{PlacementGrid, GRID_CELL_SIZE, GameRoot, GRID_CELL_AMOUNT_HEIGHT, GRID_CELL_AMOUNT_WIDTH};
use super::grid::{Grid, CellInUse};
use super::temp::*;
use super::wires::Wire;
pub struct ComponentSetupPlugin;

impl Plugin for ComponentSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlaceComponentEvent>()
            .init_resource::<Grid>()
            .add_exit_system(crate::GameState::InGame, clear_grid)
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

fn clear_grid(mut grid: ResMut<Grid>) {
    let grid = grid.as_mut();
    grid.fill(&GridPos(UVec2::new(0,0)), &UVec2::new(GRID_CELL_AMOUNT_WIDTH as u32, GRID_CELL_AMOUNT_HEIGHT as u32), CellInUse::Free);
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
        match source {
            Components::WirePiece(_) => Self::WirePiece(Wire { grid_pos: pos.0, ..Default::default()}),
            Components::GateNot(_) => Self::GateNot(GateNot { grid_pos: pos.0, ..Default::default()}),
            Components::GateAnd(_) => Self::GateAnd(GateAnd { grid_pos: pos.0, ..Default::default()}),
            Components::SignalCopy(_) => Self::SignalCopy(SignalCopy { grid_pos: pos.0, ..Default::default()}),
            Components::SignalPassthrough(_) => Self::SignalPassthrough(SignalPassthrough { grid_pos: pos.0, ..Default::default()}),
        }
    }

    pub fn get_sprite_index(&self, texture_atlas: &TextureAtlas) -> usize {
        let s = self.get_sprite_name();
        match texture_atlas.get_texture_index(&Handle::weak(s.into())) {
            Some(idx) => idx,
            None =>  {
                panic!("Attempted to load none existent texture {}", s);
            }
        }
    }

    pub fn get_sprite_name(&self) -> &str {
        let s = match self {
            Components::WirePiece(_) => "wire_left_right",
            Components::GateNot(_) => "gate_not",
            Components::GateAnd(_) => "gate_and",
            Components::SignalCopy(_) => "signal_copy",
            Components::SignalPassthrough(_) => "signal_passthrough",
        };
        s
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            Components::WirePiece(_) => Vec2::splat(32.0),
            Components::GateNot(_) => Vec2::splat(32.0),
            Components::GateAnd(_) => Vec2::splat(64.0),
            Components::SignalCopy(_) => Vec2::new(32.0,64.0),
            Components::SignalPassthrough(_) => Vec2::splat(32.0),
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
    mut place_ev: EventReader<PlaceComponentEvent>,
    mut grid_data: ResMut<Grid>,
    placement_grid: Query<(&Sprite, &Transform, &Size, With<PlacementGrid>)>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
) {
    let atlas = atlases.get(&main_atlas.handle).unwrap();
    let grid = placement_grid.single();
    let size = grid.2;
    let grid_bottom_left = grid.1.translation.truncate() - (size.0 * 0.5);
    for placement in place_ev.iter() {
        let component = Components::create_default(&placement.1, &placement.0);
        let grid_size = component.get_grid_size();
        if grid_data.can_fit(&placement.0, &grid_size) == false {
            continue;
        }
        grid_data.fill(&placement.0, &grid_size, CellInUse::Occupied);
        let mut sprite = TextureAtlasSprite::new(placement.1.get_sprite_index(atlas));
        sprite.anchor = Anchor::BottomLeft;
        commands.spawn((SpriteSheetBundle {
            sprite: sprite,
            transform: Transform {
                translation: calc_grid_pos(&grid_bottom_left, &placement.0).extend(11.0),
                //scale: Vec3::splat(2.0),
                ..Default::default()
            },
            texture_atlas: main_atlas.handle.clone(),
            ..Default::default()
        },
        GameRoot,
        component,
        Name::new(format!("Component - {}", (&placement).1.get_sprite_name())),
    )); // TODO! Component for board components and functionality
    }
}

fn calc_grid_pos(grid_bottom_left: &Vec2, pos_in_grid: &GridPos) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.0.as_vec2() * GRID_CELL_SIZE);
    pos
}