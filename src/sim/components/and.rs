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

/// A 'And' gate component which should do typical AND behaviour, consider 100 ON, anything else OFF\
/// No connection defaults to 0 hence off
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct GateAnd {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<GateAndPorts, ComponentPortData>,
}

impl GridComponent for GateAnd {
    // And gate basic funtionality
    fn tick(&mut self, _: [usize; 2], _: usize, _: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input_a = self.ports[GateAndPorts::InputA].get();
        let input_b = self.ports[GateAndPorts::InputA].get();
        if input_a >= 255 && input_b >= 255 {
            self.ports[GateAndPorts::Output].set(255);

        } else {
            self.ports[GateAndPorts::Output].set(0);
        }
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

    fn gui_options(&mut self, _: &mut egui::Ui) {}
}

impl GateAnd {
    pub const CONST_PORTS: EnumMap<GateAndPorts, ([usize; 2], Side)> = EnumMap::from_array([
        ([0, 1], Side::Left),
        ([1, 1], Side::Right),
        ([0, 0], Side::Left),
    ]);
}

#[derive(Debug, Enum)]
pub enum GateAndPorts {
    InputA,
    Output,
    InputB,
}
