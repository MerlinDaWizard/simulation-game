use bevy::reflect::{Reflect, FromReflect};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

/// A 'Not' gate component which should invert the input posting it as the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct GateNot {
}

impl GridComponent for GateNot {
    // Not gate basic funtionality
    fn tick(&mut self, own_pos: &(usize,usize), grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        // Should invert the input and post it as the output
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }
}