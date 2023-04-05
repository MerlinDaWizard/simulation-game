use std::sync::{Arc, atomic::AtomicU8};
use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use enum_map::{EnumMap, Enum};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, helpers::Side};

/// More of a debug component, not sure if it will really be need in final program\
/// Copy input into both outputs - Very similar to passthrough
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalCopy {
    #[reflect(ignore)] #[serde(skip)]
    ports: EnumMap<SignalCopyPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for SignalCopy {
    fn tick(&mut self, own_pos: &[usize; 2], grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // output = input
        todo!()
    }

    fn build(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&self, own_pos: &[usize; 2], sim_data: &SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        ()
    }
    
    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        return Self::CONST_PORTS.values().collect()
    }
}


impl SignalCopy {
    pub const CONST_PORTS: EnumMap<SignalCopyPorts, ([usize; 2], Side)> = EnumMap::from_array([ ([0,1], Side::Left), ([0,1], Side::Right), ([0,0], Side::Right) ]);
}

#[derive(Debug, Enum)]
pub enum SignalCopyPorts {
    Input,
    OutputA,
    OutputB,
}