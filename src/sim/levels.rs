use std::{fs::File, path::PathBuf};

use bevy::{prelude::{Resource, Res, ResMut, Commands, Entity, Transform, Query, Assets, With, DespawnRecursiveExt, EventReader}, reflect::Reflect, utils::HashMap, sprite::{TextureAtlas, Sprite}};
use serde::{Deserialize, Serialize};
use crate::{game::{GridSize, PlacementGridEntity}, MainTextureAtlas, components::placement::{GridLink, Size}};

use super::{model::{ComponentGrid, SimulationData, CellState}, port_grid::PortGrid};

/// Stores the relevant level state, these should be kept when levels are loaded.\
/// As opposed to [LevelData], this stores also the start positions & grid size
#[derive(Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct LevelDataLoad {
    /// Name of the level.
    pub name: String,
    /// Descrption of the level.
    pub desc: String,
    /// Grid Size, Typically should be 7x7, Undefined behaviour otherwise.
    pub grid_size: [usize; 2],
    // Using bevy hashmap which simply re-exports hashbrown (I believe)
    /// Inputs (relative to game grid)
    pub provided_inputs: HashMap<String, Vec<u8>>,
    /// Expected outputs (relative to game grid)
    pub expected_outputs: HashMap<String, Vec<u8>>,
    /// Starting state, should always be loaded before any save.
    pub start: ComponentGrid,
}

#[derive(Debug, Clone, Resource, Reflect)]
pub struct LevelData {
    /// Name of the level.
    pub name: String,
    /// Descrption of the level.
    pub desc: String,
    /// Inputs (relative to game grid)
    pub provided_inputs: HashMap<String, Vec<u8>>,
    /// Expected outputs (relative to game grid)
    pub expected_outputs: HashMap<String, Vec<u8>>,
}

impl LevelData {
    pub fn from_load(load: LevelDataLoad) -> LevelData {
        LevelData {
            name: load.name,
            desc: load.desc,
            provided_inputs: load.provided_inputs,
            expected_outputs: load.expected_outputs
        }
    }
}

pub fn load_level_listener(
    mut listener: EventReader<LoadLevelEvent>,
    mut commands: Commands,
    mut sim_data_res: ResMut<SimulationData>,
    mut grid_size: ResMut<GridSize>,
    placement_grid: Query<(&Sprite, &Transform, &Size), With<PlacementGridEntity>>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
    despawns: Query<Entity, With<GridLink>>,
) {
    for event in listener.iter() {

        for entity in despawns.iter() {
            commands.entity(entity).despawn_recursive();
        }
        
        let file = File::open(event.0.clone())
            .expect("Could not find level file");
        let level_data_load: LevelDataLoad = serde_json::from_reader(file).expect("Could not parse level");
        let grid = placement_grid.single();
        let size = grid.2;
        let grid_bottom_left = grid.1.translation.truncate() - (size.0.as_vec2() * 0.5);

        grid_size.0 = level_data_load.grid_size.clone();

        let mut sim_data = SimulationData {
            grid: ComponentGrid { grid: vec![vec![ CellState::Empty; grid_size.0[1]]; grid_size.0[0]] },
            port_grid: PortGrid::new_with_size(grid_size.0[1],  grid_size.0[0]),
        };

        let atlas = atlases.get(&main_atlas.handle).unwrap();

        for x in 0..level_data_load.start.grid.len() {
            for y in 0..level_data_load.start.grid[x].len() {
                if let CellState::Real(_, component) = level_data_load.start.grid[x][y].clone() {
                    sim_data.load_component(&mut commands, component, &grid_bottom_left, atlas, &main_atlas, &[x,y])
                }
            }
        }

        let level_data = LevelData::from_load(level_data_load);

        *sim_data_res = sim_data;
        commands.insert_resource(level_data);
    }
}

pub struct LoadLevelEvent(pub PathBuf);