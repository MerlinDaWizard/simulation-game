use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent, self, Port, Direction};

/// More of a debug component, not sure if it will really be need in final program\
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalPassthrough {
    #[reflect(ignore)]
    #[serde(skip)]
    input: Port,
    #[serde(skip)]
    #[reflect(ignore)]
    output: Port,
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

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }

    fn ports(&self) -> Vec<([usize; 2], model::Direction)> {
        vec![
            ([0,0], Direction::Right),
            ([0,0], Direction::Left)
        ]
    }

    fn ports_link(&mut self) -> Vec<([usize; 2], model::Direction, &mut model::Port)> {
        vec![
            ([0,0], Direction::Right, &mut self.output),
            ([0,0], Direction::Left, &mut self.input)
        ]
    }
}