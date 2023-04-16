use std::sync::{Arc, atomic::AtomicU8};
use strum::IntoEnumIterator;
use bevy::{prelude::*};

use super::{model::{SimulationData, CellState, GridComponent, Component}, helpers::{self, Side}, port_grid::PortGrid, components::Wire};

pub const PORT_UNWRAP_MSG: &str = "No Arc on port connection during tick";
pub struct SimRunPlugin;

impl Plugin for SimRunPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RunType>()
        .init_resource::<TickCounter>()
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

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct TickCounter(usize);

impl TickCounter {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

/// Variable interval tick event for the simulation. Can run multiple times per frame if necessary.\
/// Contains check against [RunType]\
/// An exclusive system meaning that components can do ANYTHING IN THE WORLD in their tick method
pub fn sim_tick(
    world: &mut World
) {
    let tick_count = world.get_resource::<TickCounter>().unwrap().0;
    let mut run_type = world.get_resource_mut::<RunType>().unwrap();
    // Stop tick action depending on [RunType]
    match run_type.as_mut() {
        RunType::None => {
            error!("Active but no runtype");
            return;
        },
        RunType::Step(remaining) => {
            *remaining -= 1;
            if *remaining <= 0 {
                world.insert_resource(NextState(Some(SimState::Paused)));
            }
        },
        RunType::Continuous => {},
    }

    let raw_world = world as *mut World;
    unsafe  {
        let mut sim_data = raw_world.as_mut().unwrap().get_resource_mut::<SimulationData>().unwrap();
        let grid = &mut sim_data.grid.grid;
        // Iterate through all components and tick them.
        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                match &mut grid[x][y] {
                    CellState::Empty => {continue;},
                    CellState::Reference(_) => {continue;},
                    CellState::Real(_, comp) => {
                        comp.tick([x,y], tick_count, raw_world.as_mut().unwrap());
                    },
                }
            }
        }
    }

    // Increment tick counter
    world.get_resource_mut::<TickCounter>().unwrap().increment();
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
            let mut cell = &mut grid[x][y];
            //let mut cell = std::mem::replace(&mut grid[x][y], CellState::Empty); // TODO: Revamp this code so I am never replacing the cell, this pains me to do.
            if let CellState::Real(_, component) = &mut cell {
                component.build();
                // We do these map shenanigans as the .ports returns the [usize; 2]  REFERENCES, this meant that component had to be kept in scope even after the .ports has finished
                // As a solution we just convert a Vec<&[usize; 2], Side> into a Vec<[usize; 2], Side>
                let ports: Vec<([usize; 2], Side)> = component.ports().iter().map(|(pos, side)| {(pos.clone(), side.clone())}).collect();
                for (offset, side) in ports {
                    let position = [x+offset[0], y+offset[1]];
                    if let Some(side_pos) = helpers::combine_offset(&position, &side.as_offset()) {
                        let shared = Arc::new(AtomicU8::new(0));
                        //dbg!("Orginal Pos:");
                        //dbg!(&side_pos);
                        flood_fill(grid, port_grid, shared, side_pos, side.reverse(), &mut checked_grid);
                    } else {continue;}

                }
            }
            //grid[x][y] = cell;
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
    has_propagated: &mut Vec<Vec<bool>>
) {
    let mut call_on_sides: Vec<Side> = Vec::new();
    if let Some(c) = grid.get_mut(position[0]) {
        if let Some(cell) = c.get_mut(position[1]) {
            match cell {
                CellState::Empty => {return;},
                CellState::Reference(real_pos) => {
                    let real_pos = real_pos.clone();
                    if let CellState::Real(_,c) = &mut grid[real_pos[0]][real_pos[1]] {
                        c.set_port(get_difference(&position, &real_pos), origin_side, source_arc.clone()).expect("Portgrid & component grid missmatch");
                    }
                },
                CellState::Real(_, component) => {
                    if let Component::WirePiece(piece) = component {
                        if has_propagated[position[0]][position[1]] == false {
                            has_propagated[position[0]][position[1]] = true;
                            //dbg!(piece.connected_sides);
                            let connected_sides = piece.connected_sides.iter().filter(|(_, connected)| {**connected}).map(|(side, _)| {side});
                            call_on_sides.extend(connected_sides);
                        }
                    } else {
                        if let Ok(p) = port_grid.get_mut_port_inside(&position, origin_side) {
                            if let Some(port) = p.as_mut() {
                                port.checked = true;
                                component.set_port([0,0], origin_side, source_arc.clone()).expect("Component grid and port grid missmatch");
                            }
                        }
                    }
                },
            }
        }
    }

    //dbg!(&call_on_sides);
    for dir in call_on_sides {
        if let Some(new_p) = helpers::combine_offset(&position, &dir.as_offset()) {
            flood_fill(grid, port_grid, source_arc.clone(), new_p, dir.reverse(), has_propagated);
        }
    }
}

fn get_difference(larger: &[usize; 2], smaller: &[usize; 2]) -> [usize; 2] {
    [larger[0]-smaller[0], larger[1]-smaller[1]]
}