use std::{cell::{RefCell, Cell}, sync::{atomic::{AtomicUsize, AtomicU8}, Arc}};
// Theres afew ways to store the ports, One way would be to have a 2d grid with each cell containing the top and left edges, these would be a  [Option<Port>; 2]
// Array of 2 due to having components feed directly into each other,
// this would make it fast and easy to build simulation however slightly annoying due to all the references and perhaps Mutex, Rwlock needed
// Alternatively whenever a wire updates we could iterate through each component then iterate through each port (implemented by trait)
// The trait impl would store the offset from bottom left (origin) of the component aswell as the direction it comes from
// We could then reverse all those directions then index the wire using these. Filter WirePos == PortDestinationPos
// Would require extra code for P2P (port to port) connections not through a wire
// Can already get a mental map for this

// Improvement: We add a new 'reference' component which just redirects any calls onto the actual cell the component is in.
// This means that instead of going through every component for ports we just go through the ones which are adjacent
use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;
use crate::sim::components::*;
use super::{port_grid::{PortGrid, Port}, helpers::Side};

/// The type  that wires should store using
type WireDataType = u16;

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct SimulationData {
    pub grid: ComponentGrid,
    #[reflect(ignore)]
    pub port_grid: PortGrid,
}

impl SimulationData {
    pub fn add_default_component(&mut self, dummy_component: DummyComponent, position: [usize;2], sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) -> Result<(), PortGridError> {
        let mut component = dummy_component.build_default();
        component.on_place(&position, self, sprite, atlas);
        self.grid.add_default_component(component, &position)?;
        self.port_grid.modify_bulk(Some(Port(None)), dummy_component.ports(), &position);
        Ok(())
    }

    pub fn remove_component(&mut self, component: DummyComponent, position: [usize; 2]) -> Result<(), PortGridError> {
        self.grid.remove_component(component, &position);
        self.port_grid.modify_bulk(None, component.ports(), &position);
        Ok(())
    }
}

/// The 2d grid of components
#[derive(Debug, Default, Reflect)]
pub struct ComponentGrid {
    pub grid: Vec<Vec<CellState>>,
}

pub enum PortGridError {
    CantFit
}

impl ComponentGrid {
    pub fn can_fit(&self, position: &[usize; 2], size: &[usize; 2]) -> bool {
        for x in position[0]..(position[0] + size[0]) {
            let row = match self.grid.get(x) {
                Some(r) => r,
                None => {
                    return false;
                }
            };
            for y in position[1]..(position[1] + size[1]) {
                match row.get(y) {
                    None => {return false;}
                    Some(c) => {
                        match c {
                            CellState::Empty => {},
                            CellState::Reference(_) => {return false},
                            CellState::Real(_) => {return false},
                        }
                    }
                }
            }
        }
        return true;
    }
    /// Check if component can fit and place if possible
    pub fn add_default_component(&mut self, component: Component, position: &[usize; 2]) -> Result<(), PortGridError> {
        if !self.can_fit(position, &component.dummy().get_grid_size()) {return Err(PortGridError::CantFit)}

        self.place_component(component, position);
        Ok(())
    }

    /// Place a component in the grid, does not perform any overlap checks, these are done elsewhere. See [`Self::add_default_component()`]
    fn place_component(&mut self, component: Component, position: &[usize; 2]) {
        let component_size = component.dummy().get_grid_size();
        let mut first = true; // Used to determin if to insert a real component or a grid reference
        for i in position[0]..(position[0] + component_size[0]) {
            for j in position[1]..(position[1] + component_size[1]) {
                match first {
                    true => {
                        first = false;
                        self.grid[i][j] = CellState::Real(component.clone());
                    },
                    false => {self.grid[i][j] = CellState::Reference(position.clone())}
                }
            }
        }
    }

    fn remove_component(&mut self, component: DummyComponent, position: &[usize; 2]) {
        let component_size = component.get_grid_size();
        for i in position[0]..(position[0] + component_size[0]) {
            for j in position[1]..(position[1] + component_size[1]) {
                self.grid[i][j] = CellState::Empty;
            }
        }
    }
}
/// Contains Marker varients to pass around when wanting to create or refer to a type without all the data attached
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, Clone, Copy)]
pub enum DummyComponent {
    WirePiece,
    GateNot,
    GateAnd,
    SignalCopy,
    SignalPassthrough,
    Counter,
}

#[derive(Debug, Clone, Reflect, FromReflect)]
pub enum CellState {
    /// An empty cell
    Empty,
    /// Stores the grid co-ordinates for master component
    Reference([usize; 2]),
    /// Contains a master component
    Real(Component)
}
/// Each possible component for a given cell\
/// If the cell is empty it should be expressed in the parent Option<> instead of here.
#[derive(Debug, Clone, Reflect, FromReflect)]
#[enum_dispatch]
pub enum Component {
    WirePiece(Wire),
    GateNot(GateNot),
    GateAnd(GateAnd),
    SignalCopy(SignalCopy),
    SignalPassthrough(SignalPassthrough),
    Counter(Counter),
}

/// A struct to contain any (for player only) visual events to help with understanding whats happening\
/// E.g. [VisualEventType::Flash] to make a component light up
pub struct VisualEvent {
    pub placement: (f32, f32),
    pub event_type: VisualEventType
}

/// The type of visual event to create.\
/// E.g. [VisualEventType::ElectricFizz] for a nice blue lightning particle effect
pub enum VisualEventType {
    ElectricFizz,
    Flash,
    Fire,
}

/// A struct to contain any audio events producted by the simulation\
/// E.g. Component Beeps
pub struct AudioEvent {
    pub path: &'static str,
    pub volume: f64,

}

/// adsasd
#[enum_dispatch(Component)]
pub trait GridComponent {
    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas);
    /// When assembling the simulation + setting up, what should it reset / alter\
    /// E.g. Any wire must flood fill to find its neightbours for the wire graph
    fn build(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData);

    /// Should run the update on the component using itself
    fn tick(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>);

    fn ports(&self) -> Vec<&([usize; 2], Side)>;

}
