use std::sync::{Arc, atomic::AtomicU8};

use bevy::prelude::*;
use bevy_asset_loader::prelude::LoadingStateAppExt;

use super::{model::{SimulationData, CellState, GridComponent, ComponentGrid}, helpers, port_grid::PortGrid};

pub struct SimRunPlugin;

impl Plugin for SimRunPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                CoreSet::FixedUpdate,
                AfterFixedUpdate::FixedUpdateFlush,
                CoreSet::Update,
            ).chain()
        ).add_system(apply_system_buffers.in_base_set(AfterFixedUpdate::FixedUpdateFlush))
        .insert_resource(FixedTime::new_from_secs(0.5))
        .add_system(sim_tick.in_schedule(CoreSchedule::FixedUpdate).run_if(in_state(SimState::Active)))
        .init_resource::<RunType>();

    }
}


#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
pub enum AfterFixedUpdate {
    FixedUpdateFlush
}


/// Simulation State
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum SimState {
    #[default]
    Halted,
    Paused,
    Active,
}

#[derive(Debug, Resource, Clone, Default)]
pub enum RunType {
    #[default]
    None,
    Step(u32),
    Continuous,
}

/// Variable interval tick event for the simulation. Can run multiple times per frame if necessary.\
/// Contains check against [RunType]
pub fn sim_tick(
    mut commands: Commands,
    mut run_type: ResMut<RunType>,
    mut sim_data: ResMut<SimulationData>,
) {
    // Stop tick action depending on [RunType]
    match run_type.as_mut() {
        RunType::None => {return;},
        RunType::Step(remaining) => {
            *remaining -= 1;
            if *remaining <= 0 {
                commands.insert_resource(NextState(Some(SimState::Paused)));
            }
        },
        RunType::Continuous => {},
    }
}

pub fn build_simulation(
    mut sim_data: ResMut<SimulationData>
) {
    let mut sim_data = sim_data.as_mut();
    let grid = &mut sim_data.grid.grid;
    let port_grid = &mut sim_data.port_grid;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let cell = std::mem::replace(&mut grid[x][y], CellState::Empty); // TODO: Revamp this code so I am never replacing the cell, this pains me to do.
            if let CellState::Real(_, component) = cell {
                for (offset, side) in component.ports() {

                    let position = [x+offset[0], y+offset[1]];
                    let port = port_grid.get_mut_port(&position, *side).expect("Portgrid & component grid missmatch");
                    // Unwrap it a bit mroe
                    let port = port.as_mut().expect("Portgrid & component grid missmatch");
                    // Exit if already checked
                    if port.checked == true {continue;}
                    let shared = Arc::new(AtomicU8::new(0));
                    port.val = Some(shared.clone());

                    if let Some(new_p) = helpers::combine_offset(&position, &side.as_offset()) {
                        flood_fill(grid, port_grid, shared, new_p);
                    } else {continue;}

                }
            }
            *grid[x][y] = cell
        }
    }
}

pub fn flood_fill(
    grid: &mut Vec<Vec<CellState>>,
    port_grid: &mut PortGrid,
    source_arc: Arc<AtomicU8>,
    position: [usize; 2],
) {
    for i in grid {
        for j in i {
            if let CellState::Real(_, c) = j {
                
            }
        }
    }
}