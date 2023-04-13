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
    for (x, a) in sim_data.grid.grid.iter_mut().enumerate() {
        for (y, b) in a.iter_mut().enumerate() {
            if let CellState::Real(_, component) = b {
                for (offset, side) in component.ports() {
                    let position = [x+offset[0], y+offset[1]];
                    let shared = Arc::new(AtomicU8::new(0));
                    
                    {
                        let port = sim_data.port_grid.get_mut_port(&position, *side).expect("Portgrid & component grid missmatch");
                        // Unwrap it a bit mroe
                        let port = port.as_mut().expect("Portgrid & component grid missmatch");
                        // Exit if already checked
                        if port.checked == true {continue}
                        let shared = Arc::new(AtomicU8::new(0));
                        port.val = Some(shared.clone());
                    }
                    if let Some(new_p) = helpers::combine_offset(&position, &side.as_offset()) {
                        unsafe{flood_fill(sim_data as *mut SimulationData, shared, new_p);}
                    } else {continue;}

                }
            }
        }
    }
}

pub fn flood_fill(
    mut sim_data: *mut SimulationData,
    source_arc: Arc<AtomicU8>,
    position: [usize; 2],
) {
    
}