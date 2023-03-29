use bevy::reflect::{Reflect, FromReflect};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent};

/// More of a debug component, not sure if it will really be need in final program\
/// Copy input into both outputs - Very similar to passthrough
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalCopy {

}

impl GridComponent for SignalCopy {
    fn tick(&mut self, own_pos: &(usize, usize), grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // output = input
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }
}