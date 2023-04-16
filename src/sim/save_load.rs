use std::{path::PathBuf, fs::File, io::{Read, Write, BufReader}};
use base64::{write::EncoderWriter, read::DecoderReader};
use bevy::{prelude::*, ecs::system::SystemState};
use flate2::{write::ZlibEncoder, Compression, bufread::ZlibDecoder};
use serde::{Deserialize, Serialize};
use crate::{game::{GridSize, PlacementGridEntity}, MainTextureAtlas, components::placement::{Size, GridLink}, GameState};
use super::{run::SimState, model::{SimulationData, ComponentGrid, CellState}, port_grid::PortGrid};
pub struct SimLoadPlugin;

impl Plugin for SimLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveEvent>()
        .add_event::<LoadEvent>()
        .add_system(save_listener.run_if(in_state(GameState::InGame)))
        .add_system(load_listener.run_if(in_state(GameState::InGame)));
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LevelData {
    pub grid_size: GridSize,
    pub component_grid: ComponentGrid,
}

impl FromWorld for LevelData {
    fn from_world(world: &mut World) -> Self {
        let sim_state = world.get_resource::<State<SimState>>().unwrap().0;
        if sim_state != SimState::Halted {
            panic!("Must be halted to to save world");
        }

        let sim_data = world.get_resource::<SimulationData>().expect("Cant find SimulationData to save");
        let component_grid = sim_data.grid.clone();

        LevelData {
            grid_size: world.get_resource::<GridSize>().unwrap().clone(),
            component_grid: component_grid,
        }
    }
}

impl LevelData {
    pub fn create_world(mut self, commands: &mut Commands, atlas: &TextureAtlas, main_atlas: &MainTextureAtlas, placement_grid: &Query<(&Sprite, &Transform, &Size), With<PlacementGridEntity>>) -> (SimulationData, GridSize) {
        let grid = placement_grid.single();
        let size = grid.2;
        let grid_bottom_left = grid.1.translation.truncate() - (size.0.as_vec2() * 0.5);
        
        let grid_size = self.grid_size;
        let mut sim_data = SimulationData {
            grid: ComponentGrid { grid: vec![vec![ CellState::Empty; grid_size.0[1]]; grid_size.0[0]] },
            port_grid: PortGrid::new_with_size(grid_size.0[1],  grid_size.0[0]),
        };

        for x in 0..self.component_grid.grid.len() {
            for y in 0..self.component_grid.grid[x].len() {
                if let CellState::Real(_, component) = self.component_grid.grid[x][y].clone() {
                    sim_data.load_component(commands, component, &grid_bottom_left, atlas, main_atlas, &[x,y])
                }
            }
        }
        (sim_data, grid_size)
    }
}

pub struct SaveEvent(pub PathBuf);
pub struct LoadEvent(pub PathBuf);

/// Listens for [SaveEvent] and saves the simulation data to the corrisponding path buf.
fn save_listener(
    world: &mut World,
    listener: &mut SystemState<EventReader<SaveEvent>>,
) {
    let mut paths = Vec::new();
    let mut listener = listener.get(world);
    for ev in listener.iter() { // Two steps to make borrow checker hpapy, will normally only be 1 save event per frame anyway.
        paths.push(ev.0.clone());
    }
    
    for path in paths {
        let data = LevelData::from_world(world);
        let vec = serde_json::to_vec(&data).expect("Couldn't write to file :(");
        let mut uncompressed_file = File::create("data/levels/uncompressed.json").expect("Cannot create / recreate file");
        uncompressed_file.write(&vec).expect("Could not write file");
        let compressed_file = File::create(path).expect("Cannot create / recreate file");
        let mut encoder = ZlibEncoder::new(EncoderWriter::new(compressed_file, &base64::prelude::BASE64_STANDARD_NO_PAD), Compression::new(9));
        encoder.write(&vec).expect("Could not compress save");
        encoder.finish().expect("Could not write file");
    }
}

fn load_listener(
    mut listener: EventReader<LoadEvent>,
    mut commands: Commands,
    mut sim_data: ResMut<SimulationData>,
    mut size: ResMut<GridSize>,
    placement_grid: Query<(&Sprite, &Transform, &Size), With<PlacementGridEntity>>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
    despawns: Query<Entity, With<GridLink>>
) {
    for ev in listener.iter() {
        // Clear all pre-existing sprites.
        for entity in despawns.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let file = File::open(&ev.0).expect("Could not find level file");
        // First use the buffered base64 decoder, create a buffered reader using that
        // Then use that with Zlib to decode that into the json
        let reader = ZlibDecoder::new(BufReader::new(DecoderReader::new(file, &base64::prelude::BASE64_STANDARD_NO_PAD)));

        let level_data: LevelData = serde_json::from_reader(reader).expect("Could not parse level");
        // Recreate [SimulationData] etc.
        let (new_sim_data, new_size) = level_data.create_world(&mut commands, atlases.get(&main_atlas.handle).unwrap(), main_atlas.as_ref(), &placement_grid);
        *sim_data = new_sim_data;
        *size = new_size;
    }
}
