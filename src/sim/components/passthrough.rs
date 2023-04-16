use crate::sim::{
    helpers::Side,
    model::{AudioEvent, GridComponent, SimulationData, VisualEvent, ComponentPortData},
};
use bevy::{
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite}, prelude::World,
};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU8, Arc};

/// More of a debug component, not sure if it will really be need in final program\
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct SignalPassthrough {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<SignalPassthroughPorts, ComponentPortData>,
}

impl GridComponent for SignalPassthrough {
    // Not gate basic funtionality
    fn tick(&mut self, _: [usize; 2], _: usize, _: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input = self.ports[SignalPassthroughPorts::Input].get();
        self.ports[SignalPassthroughPorts::Output].set(input);
        (Vec::new(), Vec::new())
    }


    fn build(&mut self) {}

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
                self.ports[varient].set_link(Some(set_to));
                return Ok(());
            }
        }
        Err(())
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
