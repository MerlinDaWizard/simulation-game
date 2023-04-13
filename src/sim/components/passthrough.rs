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
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalPassthrough {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<SignalPassthroughPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for SignalPassthrough {
    // Not gate basic funtionality
    fn tick(
        &mut self,
        _own_pos: &[usize; 2],
        _grid: &mut SimulationData,
    ) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // Should invert the input and post it as the output
        // output = (100-input)
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

impl SignalPassthrough {
    pub const CONST_PORTS: EnumMap<SignalPassthroughPorts, ([usize; 2], Side)> =
        EnumMap::from_array([([0, 0], Side::Left), ([0, 0], Side::Right)]);
}

#[derive(Debug, Enum)]
pub enum SignalPassthroughPorts {
    Input,
    Output,
}
