use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

/// A 'And' gate component which should do typical AND behaviour, consider 100 ON, anything else OFF\
/// No connection defaults to 0 hence off
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct GateAnd {
    #[serde(skip)] input_A: Option<usize>,
    #[serde(skip)] input_B: Option<usize>,
    #[serde(skip)] output: Option<usize>,
}

impl GridComponent for GateAnd {
    // And gate basic funtionality
    fn tick(&mut self, own_pos: &(usize, usize), grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // A && B = C
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }
}