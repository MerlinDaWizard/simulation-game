use std::sync::{Arc, atomic::AtomicU8};
use strum::IntoEnumIterator;
use bevy::prelude::*;

use super::{model::{SimulationData, CellState, GridComponent, Component}, helpers::{self, Side}, port_grid::PortGrid, components::Wire};

pub struct SimRunPlugin;

impl Plugin for SimRunPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RunType>()
        .add_state::<SimState>()
        .configure_sets(
            (
                CoreSet::FixedUpdate,
                AfterFixedUpdate::FixedUpdateFlush,
                CoreSet::Update,
            ).chain()
        ).add_system(apply_system_buffers.in_base_set(AfterFixedUpdate::FixedUpdateFlush))
        .insert_resource(FixedTime::new_from_secs(0.5))
        .add_system(sim_tick.in_schedule(CoreSchedule::FixedUpdate).run_if(in_state(SimState::Active)))
        //.add_system(build_simulation.in_schedule(OnEnter(SimState::Building))) // Should learn more about the ECS to set the ordering of this stuff better to prevent 1 frame delays
        .add_systems((build_simulation, build_to_active).chain().in_schedule(OnEnter(SimState::Building)));
    }
}

fn build_to_active(mut commands: Commands) {
    println!("MOVED TO ACTIVE");
    println!("MOVED TO ACTIVE");
    commands.insert_resource(NextState(Some(SimState::Active)));
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
    Building,
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
        RunType::None => {
            error!("Active but no runtype");
            return;
        },
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
    println!("BUILDING");
    let sim_data = sim_data.as_mut();
    let grid = &mut sim_data.grid.grid;
    let port_grid = &mut sim_data.port_grid;
    let mut checked_grid = vec![vec![false; grid[0].len()]; grid.len()];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            let cell = std::mem::replace(&mut grid[x][y], CellState::Empty); // TODO: Revamp this code so I am never replacing the cell, this pains me to do.
            if let CellState::Real(_, component) = &cell {
                for (offset, side) in component.ports() {

                    let position = [x+offset[0], y+offset[1]];
                    dbg!(position);
                    dbg!(side);
                    dbg!(port_grid.get_sides(&position));
                    if let Some(side_pos) = helpers::combine_offset(&position, &side.as_offset()) {
                        let port = port_grid.get_mut_port(&side_pos, side.reverse()).expect("Portgrid & component grid missmatch");
                        dbg!(&port);
                        // Unwrap it a bit mroe
                        let port = port.as_mut().expect("Portgrid & component grid missmatch");
                        // Exit if already checked
                        if port.checked == true {continue;}
                        let shared = Arc::new(AtomicU8::new(0));
                        port.val = Some(shared.clone());

                        flood_fill(grid, port_grid, shared, side_pos, side.reverse(), &mut checked_grid);
                    } else {continue;}

                }
            }
            grid[x][y] = cell;
        }
    }
    dbg!(grid);
}

pub fn flood_fill(
    grid: &mut Vec<Vec<CellState>>,
    port_grid: &mut PortGrid,
    source_arc: Arc<AtomicU8>,
    position: [usize; 2],
    origin_side: Side,
    checked_grid: &mut Vec<Vec<bool>>
) {
    // Found a component port
    if let Ok(bounds) = port_grid.get_mut_port(&position, origin_side.reverse()) {
        if let Some(port) = bounds {
            port.checked = true;
            port.val = Some(source_arc.clone());
            checked_grid[position[0]][position[1]] = true;
            let cell = &mut grid[position[0]][position[1]];
            match cell {
                CellState::Empty => panic!("Portgrid & component grid missmatch"),
                CellState::Reference(real_pos) => {
                    let real_pos = real_pos.clone();
                    if let CellState::Real(_,c) = &mut grid[real_pos[0]][real_pos[1]] {
                        c.set_port(get_difference(&position, &real_pos), origin_side, source_arc).expect("Portgrid & component grid missmatch");
                    }
                },
                CellState::Real(_, c) => {
                    dbg!(&position);
                    dbg!(origin_side);
                    dbg!(&c);
                    c.set_port([0,0], origin_side, source_arc).expect("Portgrid & component grid missmatch");
                },
            }
            return;
        }
    }

    let cell = &mut grid[position[0]][position[1]];
    if let CellState::Real(_, c) = cell {
        if let Component::WirePiece(_) = c {
            if checked_grid[position[0]][position[1]] == false {
                for dir in helpers::Side::iter() {
                    if dir == origin_side {continue} // Just dont immediatally bounce back (theoretically this wouldnt be an issue anyway)
                    if let Some(new_p) = helpers::combine_offset(&position, &dir.as_offset()) {
                        flood_fill(grid, port_grid, source_arc.clone(), new_p, dir.reverse(), checked_grid);
                    } else {continue;}
                }
            }
        }
    }

}

fn get_difference(larger: &[usize; 2], smaller: &[usize; 2]) -> [usize; 2] {
    [larger[0]-smaller[0], larger[1]-smaller[1]]
}