use std::sync::{Arc, atomic::AtomicU8};
use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use enum_map::{EnumMap, Enum};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, helpers::Side};

/// More of a debug component, not sure if it will really be need in final program\
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalPassthrough {
    #[reflect(ignore)] #[serde(skip)]
    ports: EnumMap<SignalPassthroughPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for SignalPassthrough {
    // Not gate basic funtionality
    fn tick(&mut self, own_pos: &[usize; 2], grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        // Should invert the input and post it as the output
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


impl SignalPassthrough {
    pub const CONST_PORTS: EnumMap<SignalPassthroughPorts, ([usize; 2], Side)> = EnumMap::from_array([([0,0], Side::Left), ([0,0], Side::Right)]);
}

#[derive(Debug, Enum)]
pub enum SignalPassthroughPorts {
    Input,
    Output,
}