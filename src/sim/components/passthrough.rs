use bevy::reflect::{Reflect, FromReflect};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

/// More of a debug component, not sure if it will really be need in final program\
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalPassthrough {

}

impl GridComponent for SignalPassthrough {
    // Not gate basic funtionality
    fn tick(&mut self, own_pos: &(usize,usize), grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        // Should invert the input and post it as the output
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &(usize,usize), sim_data: &mut SimulationData) {
        todo!()
    }
}