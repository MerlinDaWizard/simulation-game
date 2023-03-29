use std::fs::File;
use std::io::Read;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use iyes_loopless::prelude::{ConditionSet, AppLooplessStateExt};
use crate::MainTextureAtlas;
use crate::game::{PlacementGridEntity, GRID_CELL_SIZE, GameRoot, GridSize};
use crate::level_select::CurrentLevel;
use crate::sim::components::*;
use crate::sim::level::LevelData;
use crate::sim::model::{Component as SimComponent, SimulationData, DummyComponent as DummySimComponent};
use super::occupation_grid::{CellInUse, OccupationGrid};
pub struct ComponentSetupPlugin;

impl Plugin for ComponentSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlaceComponentEvent>()
            .init_resource::<OccupationGrid>()
            .init_resource::<SimulationData>()
            .init_resource::<GridSize>()
            .add_enter_system(crate::GameState::InGame, setup_grid)
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

fn setup_grid(
    level: Res<CurrentLevel>,


    mut occupation_grid: ResMut<OccupationGrid>,
    mut sim_data: ResMut<SimulationData>,
    mut grid_size: ResMut<GridSize>,
) {
    let mut s = String::new();
    File::open(format!("data/levels/{}.json", {level.0.unwrap()})).expect("Could not find level file").read_to_string(&mut s).unwrap();
    let level_data: LevelData = serde_json::from_str(&s).expect("Could not parse level");
    let size = GridSize([level_data.grid_width, level_data.grid_height]);
    *occupation_grid = OccupationGrid::empty_grid_from_size(&size);
    //occupation_grid.0 = OccupationGrid::empty_grid_from_size(&size);
    sim_data.wire_graph = Vec::new();
    sim_data.grid.grid = vec![vec![None; size.0[1]]; size.0[0]];
    grid_size.0 = [level_data.grid_width, level_data.grid_height];
}

fn clear_grid(mut occupation_grid: ResMut<OccupationGrid>, mut simulation_grid: ResMut<SimulationData>, grid_size: Res<GridSize>) {
    occupation_grid.fill(&[0,0], &grid_size.0, CellInUse::Free)
}

impl SimComponent {
    /// Converts it to it's dummy counterpart to get shared data
    pub fn dummy(&self) -> DummySimComponent {
        match self {
            SimComponent::WirePiece(_) => DummySimComponent::WirePiece,
            SimComponent::GateNot(_) => DummySimComponent::GateNot,
            SimComponent::GateAnd(_) => DummySimComponent::GateAnd,
            SimComponent::SignalCopy(_) => DummySimComponent::SignalCopy,
            SimComponent::SignalPassthrough(_) => DummySimComponent::SignalPassthrough,
            SimComponent::Counter(_) => DummySimComponent::Counter,
        }
    }
}

impl DummySimComponent {
    pub fn build_default(&self) -> SimComponent {
        match self {
            Self::WirePiece => SimComponent::WirePiece(Wire::default()),
            Self::GateNot => SimComponent::GateNot(GateNot::default()),
            Self::GateAnd => SimComponent::GateAnd(GateAnd::default()),
            Self::SignalCopy => SimComponent::SignalCopy(SignalCopy::default()),
            Self::SignalPassthrough => SimComponent::SignalPassthrough(SignalPassthrough::default()),
            Self::Counter => SimComponent::Counter(Counter::default()),
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
            Self::WirePiece => "wire_left_right",
            Self::GateNot => "gate_not",
            Self::GateAnd => "gate_and",
            Self::SignalCopy => "signal_copy",
            Self::SignalPassthrough => "signal_passthrough",
            Self::Counter => "signal_passthrough",
        };
        s
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            Self::WirePiece => Vec2::splat(32.0),
            Self::GateNot => Vec2::splat(32.0),
            Self::GateAnd => Vec2::splat(64.0),
            Self::SignalCopy => Vec2::new(32.0,64.0),
            Self::SignalPassthrough => Vec2::splat(32.0),
            Self::Counter => Vec2::new(32.0,64.0),
        }
    }

    pub fn get_grid_size(&self) -> [usize;2] {
        match self {
            Self::WirePiece => [1,1],
            Self::GateNot => [1,1],
            Self::GateAnd => [2,2],
            Self::SignalCopy => [1,2],
            Self::SignalPassthrough => [1,1],
            Self::SignalPassthrough => [1,1],
            Self::Counter => [1,2],
        }
    }
}

#[derive(Debug, Component)]
pub struct Size(pub UVec2);

/// A component to link between sprites (entities) and the position in the [SimulationData] resource
#[derive(Debug, Component)]
pub struct GridLink(pub [usize; 2]);


pub struct PlaceComponentEvent(pub [usize; 2], pub DummySimComponent);

fn placement_event(
    mut commands: Commands,
    mut place_ev: EventReader<PlaceComponentEvent>,
    mut occupation_grid: ResMut<OccupationGrid>,
    mut sim_data: ResMut<SimulationData>,
    placement_grid: Query<(&Sprite, &Transform, &Size), With<PlacementGridEntity>>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
) {
    let atlas = atlases.get(&main_atlas.handle).unwrap();
    let grid = placement_grid.single();
    let size = grid.2;
    let grid_bottom_left = grid.1.translation.truncate() - (size.0.as_vec2() * 0.5);
    for event in place_ev.iter() {
        
        let grid_size = event.1.get_grid_size();
        if occupation_grid.can_fit(&event.0, &grid_size) == false {
            warn!("Attempted to place component at {:?}, could not fit", {&event.0});
            eprintln!("Attempted to place component at {:?}, could not fit", {&event.0});
            continue;
        }
        occupation_grid.fill(&event.0, &grid_size, CellInUse::Occupied);
        let mut sprite = TextureAtlasSprite::new(event.1.get_sprite_index(atlas));
        sprite.anchor = Anchor::BottomLeft;
        commands.spawn((SpriteSheetBundle {
            sprite: sprite,
            transform: Transform {
                translation: calc_grid_pos(&grid_bottom_left, &UVec2::from_array([event.0[0] as u32, event.0[1] as u32])).extend(11.0),
                //scale: Vec3::splat(2.0),
                ..Default::default()
            },
            texture_atlas: main_atlas.handle.clone(),
            ..Default::default()
        },
            GameRoot,
            GridLink(event.0),
            Name::new(format!("Component - {}", event.1.get_sprite_name())),
        ));
        
        sim_data.grid.grid[event.0[0]][event.0[1]] = Some(event.1.build_default());
    }
}

fn calc_grid_pos(grid_bottom_left: &Vec2, pos_in_grid: &UVec2) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.as_vec2() * GRID_CELL_SIZE as f32);
    dbg!(pos);
    pos
}