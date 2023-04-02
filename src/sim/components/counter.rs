use std::sync::{atomic::AtomicU8, Arc};
use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use enum_map::{EnumMap, Enum};
use serde::{Serialize, Deserialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, helpers::Side};

/// A counter which whenever it reads 100 on input A and 100 on input CLK (Clock) it will increment an internal counter and post that on the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Counter {
    #[reflect(ignore)] #[serde(skip)]
    ports: EnumMap<CounterPorts, Option<Arc<AtomicU8>>>,
    #[serde(skip)]
    count: u8, // Defaults 0
}

impl GridComponent for Counter {
    fn tick(&mut self, own_pos: &[usize; 2], grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // A && B = C
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }
    
    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        return Self::CONST_PORTS.values().collect()
    }
}


impl Counter {
    pub const CONST_PORTS: EnumMap<CounterPorts, ([usize; 2], Side)> = EnumMap::from_array([ ([0,1], Side::Left), ([0,0], Side::Left), ([0,1], Side::Right) ]);
}

#[derive(Debug, Enum)]
pub enum CounterPorts {
    Input,
    InputClk,
    Output,
}