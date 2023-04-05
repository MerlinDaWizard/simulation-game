use std::sync::{atomic::AtomicU8, Arc};
use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, helpers::Side};

/// A 'Not' gate component which should invert the input posting it as the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct GateNot {
    #[reflect(ignore)] #[serde(skip)]
    ports: EnumMap<NotPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for GateNot {
    // Not gate basic funtionality
    fn tick(&mut self, own_pos: &[usize; 2], grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        // Should invert the input and post it as the output
        // output = (100-input)
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


impl GateNot {
    pub const CONST_PORTS: EnumMap<NotPorts, ([usize; 2], Side)> = EnumMap::from_array([([0,0], Side::Left), ([0,0], Side::Right)]);
}

#[derive(Debug, Enum)]
pub enum NotPorts {
    Input,
    Output,
}