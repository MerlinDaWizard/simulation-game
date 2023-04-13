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

/// More of a debug component, not sure if it will really be need in final program\
/// Copy input into both outputs - Very similar to passthrough
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalCopy {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<SignalCopyPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for SignalCopy {
    fn tick(
        &mut self,
        _own_pos: &[usize; 2],
        _grid: &mut SimulationData,
    ) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // output = input
        todo!()
    }

    fn build(&mut self, _own_pos: &[usize; 2], _sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(
        &self,
        _own_pos: &[usize; 2],
        _sim_data: &SimulationData,
        _sprite: &mut TextureAtlasSprite,
        _atlas: &TextureAtlas,
    ) {
    }

    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        return Self::CONST_PORTS.values().collect();
    }
}

impl SignalCopy {
    pub const CONST_PORTS: EnumMap<SignalCopyPorts, ([usize; 2], Side)> = EnumMap::from_array([
        ([0, 1], Side::Left),
        ([0, 1], Side::Right),
        ([0, 0], Side::Right),
    ]);
}

#[derive(Debug, Enum)]
pub enum SignalCopyPorts {
    Input,
    OutputA,
    OutputB,
}
