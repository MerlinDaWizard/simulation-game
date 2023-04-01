use bevy::reflect::{Reflect, FromReflect};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, port_grid::Side};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Wire {
}

impl GridComponent for Wire {
    // Wires do not need to tick as all communication is done intrinsically using the wire graph not graph
    fn tick(&mut self, own_pos: &(usize,usize), grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        (Vec::new(),Vec::new())
    }

    fn build(&mut self, own_pos: &(usize,usize), sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }

    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        Vec::new()
    }
}

enum ConnectionStatus {
    Connected,
    Floating,
    /// Allow disabling of certain connections to allow wires running in parallel and such
    Disabled,
}