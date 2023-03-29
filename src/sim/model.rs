use bevy::prelude::*;
use enum_dispatch::enum_dispatch;
use strum_macros::EnumIter;
use crate::sim::components::*;

/// The type  that wires should store using
type WireDataType = u16;

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct SimulationData {
    pub grid: ComponentGrid,
    pub wire_graph: Vec<WireDataType>
}

/// The 2d grid of components
#[derive(Debug, Default, Reflect)]
pub struct ComponentGrid {
    pub grid: Vec<Vec<Option<Component>>>,
}

/// Contains Marker varients to pass around when wanting to create or refer to a type without all the data attached
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, Clone)]
pub enum DummyComponent {
    WirePiece,
    GateNot,
    GateAnd,
    SignalCopy,
    SignalPassthrough,
    Counter,
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

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData);

    /// Should run the update on the component using itself
    fn tick(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>);
}