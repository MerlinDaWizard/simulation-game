use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
}