use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent, self, Port, Direction};

/// A 'And' gate component which should do typical AND behaviour, consider 100 ON, anything else OFF\
/// No connection defaults to 0 hence off
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct GateAnd {
    #[reflect(ignore)] #[serde(skip)] input_A: Port,
    #[reflect(ignore)] #[serde(skip)] input_B: Port,
    #[reflect(ignore)] #[serde(skip)] output: Port,
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

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }

    fn ports(&self) -> Vec<([usize; 2], model::Direction)> {
        vec![
            ([0,0], Direction::Left),
            ([0,1], Direction::Left),
            ([0,1], Direction::Right),
        ]
    }

    fn ports_link(&mut self) -> Vec<([usize; 2], model::Direction, &mut model::Port)> {
        vec![
            ([0,0], Direction::Left, &mut self.input_A),
            ([0,1], Direction::Left, &mut self.input_B),
            ([0,1], Direction::Right, &mut self.output),
        ]
    }
}