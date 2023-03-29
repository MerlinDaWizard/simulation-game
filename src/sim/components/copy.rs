use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent, self, Port, Direction};

/// More of a debug component, not sure if it will really be need in final program\
/// Copy input into both outputs - Very similar to passthrough
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalCopy {
    #[reflect(ignore)] #[serde(skip)] input: Port,
    #[reflect(ignore)] #[serde(skip)] output_1: Port,
    #[reflect(ignore)] #[serde(skip)] output_2: Port,
}

impl GridComponent for SignalCopy {
    fn tick(&mut self, own_pos: &(usize, usize), grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // output = input
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }

    fn ports(&self) -> Vec<([usize; 2], model::Direction)> {
        vec![
            ([0,1], Direction::Left),
            ([0,0], Direction::Right),
            ([0,1], Direction::Right),
        ]
    }

    fn ports_link(&mut self) -> Vec<([usize; 2], model::Direction, &mut model::Port)> {
        vec![
            ([0,1], Direction::Left, &mut self.input),
            ([0,0], Direction::Right, &mut self.output_1),
            ([0,1], Direction::Right, &mut self.output_2),
        ]
    }
}