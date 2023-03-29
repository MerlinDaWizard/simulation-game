use bevy::reflect::{Reflect, FromReflect};
use serde::{Serialize, Deserialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

/// A counter which whenever it reads 100 on input A and 100 on input CLK (Clock) it will increment an internal counter and post that on the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Counter {
    #[serde(skip)] input_A: Option<usize>,
    #[serde(skip)] input_clk: Option<usize>,
    #[serde(skip)] output: Option<usize>,
}

impl GridComponent for Counter {
    fn tick(&mut self, own_pos: &(usize, usize), grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // A && B = C
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }
}