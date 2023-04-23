use std::{fs::File, path::PathBuf};

use bevy::{prelude::{Resource, Res, ResMut, Commands, Entity, Transform, Query, Assets, With, DespawnRecursiveExt, EventReader}, reflect::{Reflect, FromReflect}, utils::HashMap, sprite::{TextureAtlas, Sprite}};
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

#[derive(Debug, Clone, Copy, Reflect, FromReflect)]
pub enum ResultType {
    Incorrect,
    Correct,
}

#[derive(Debug, Clone, Resource, Reflect, Default)]
pub struct SimIOPadded {
    pub output_pointer: usize,
    pub correct_so_far: bool,
    pub expected_outputs: HashMap<String, Vec<Option<u8>>>,
    pub observed_outputs: HashMap<String, Vec<Option<(u8, ResultType)>>>,
}

impl SimIOPadded {
    pub fn from_level_data(level_data: &mut LevelData) -> SimIOPadded {
        let mut expected = HashMap::new();
        let mut observed = HashMap::new();
        for (k, _) in &level_data.expected_outputs {
            expected.insert_unique_unchecked(k.clone(), Vec::new());
            observed.insert_unique_unchecked(k.clone(), Vec::new());
        }
    
        SimIOPadded {
            output_pointer: 0,
            correct_so_far: true,
            expected_outputs: expected,
            observed_outputs: observed,
        }
    }

    pub fn add_output(&mut self, level_data: &mut LevelData, _: usize, id: &str, val: Option<u8>) {
        if let Some(val) = val {
            let expected = level_data.expected_outputs[id][self.output_pointer];
            self.expected_outputs.get_mut(id).unwrap().push(Some(expected));
            self.output_pointer += 1;

            if val == expected {
                self.observed_outputs.get_mut(id).unwrap().push(Some((expected, ResultType::Correct)));
            } else {
                self.correct_so_far = false;
                self.observed_outputs.get_mut(id).unwrap().push(Some((expected, ResultType::Incorrect)));
            }

        } else {
            self.expected_outputs.get_mut(id).unwrap().push(None);
            self.observed_outputs.get_mut(id).unwrap().push(None);
        }
    }
}

impl LevelData {
    pub fn from_load(load: LevelDataLoad) -> LevelData {
        LevelData {
            name: load.name,
            desc: load.desc,
            provided_inputs: load.provided_inputs,
            expected_outputs: load.expected_outputs,
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

        let mut level_data = LevelData::from_load(level_data_load);
        let sim_io = SimIOPadded::from_level_data(&mut level_data);
        *sim_data_res = sim_data;
        commands.insert_resource(level_data);
        commands.insert_resource(sim_io);
    }
}

pub struct LoadLevelEvent(pub PathBuf);