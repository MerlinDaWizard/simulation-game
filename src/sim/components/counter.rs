use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use serde::{Serialize, Deserialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent, self, Port, Direction};

/// A counter which whenever it reads 100 on input A and 100 on input CLK (Clock) it will increment an internal counter and post that on the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Counter {
    counter: u8,
    #[reflect(ignore)] #[serde(skip)] input: Port,
    #[reflect(ignore)] #[serde(skip)] input_clk: Port,
    #[reflect(ignore)] #[serde(skip)] output: Port,
}
impl Counter {
    /// Increment the coutners insternal state, 0-255 with wrapping
    pub fn increment(&mut self) {
        self.counter.wrapping_add(1);
    }
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

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }

    fn ports(&self) -> Vec<([usize; 2], model::Direction)> {
        vec![
            ([0,0], Direction::Left),
            ([0,0], Direction::Down),
            ([0,0], Direction::Right),
            
        ]
    }

    fn ports_link(&mut self) -> Vec<([usize; 2], model::Direction, &mut model::Port)> {
        vec![
            ([0,0], Direction::Left, &mut self.input),
            ([0,0], Direction::Down, &mut self.input_clk),
            ([0,0], Direction::Right, &mut self.output),
            
        ]
    }
}