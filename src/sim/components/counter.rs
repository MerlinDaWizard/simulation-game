use crate::sim::{
    helpers::Side,
    model::{AudioEvent, GridComponent, SimulationData, VisualEvent},
};
use bevy::{
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU8, Arc};

/// A counter which whenever it reads 100 on input A and 100 on input CLK (Clock) it will increment an internal counter and post that on the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Counter {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<CounterPorts, Option<Arc<AtomicU8>>>,
    #[serde(skip)]
    count: u8, // Defaults 0
}

impl GridComponent for Counter {
    fn tick(
        &mut self,
        _own_pos: &[usize; 2],
        _grid: &mut SimulationData,
    ) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // A && B = C
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, _own_pos: &[usize; 2], _sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(
        &mut self,
        _own_pos: &[usize; 2],
        _sim_data: &SimulationData,
        _sprite: &mut TextureAtlasSprite,
        _atlas: &TextureAtlas,
    ) {
    }

    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        return Self::CONST_PORTS.values().collect();
    }

    fn set_port(&mut self, offset: [usize; 2], side: Side, set_to: Arc<AtomicU8>) -> Result<(),()> {
        for (varient, (self_offset, self_side)) in Self::CONST_PORTS.iter() {
            if offset == *self_offset && side == *self_side {
                self.ports[varient] = Some(set_to);
                return Ok(());
            }
        }
        Err(())
    }
}

impl Counter {
    pub const CONST_PORTS: EnumMap<CounterPorts, ([usize; 2], Side)> = EnumMap::from_array([
        ([0, 1], Side::Left),
        ([0, 0], Side::Left),
        ([0, 1], Side::Right),
    ]);
}

#[derive(Debug, Enum)]
pub enum CounterPorts {
    Input,
    InputClk,
    Output,
}
