use crate::sim::{
    helpers::Side,
    model::{AudioEvent, GridComponent, SimulationData, VisualEvent, ComponentPortData, DummyComponent}, interactions::UpdateComponentEvent,
};
use bevy::{
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite}, prelude::{World, EventWriter},
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
    ports: EnumMap<SignalCopyPorts, ComponentPortData>,
}

impl GridComponent for SignalCopy {
    fn tick(&mut self, _: [usize; 2], _: usize, _: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input = self.ports[SignalCopyPorts::Input].get();
        self.ports[SignalCopyPorts::OutputA].set(input);
        self.ports[SignalCopyPorts::OutputB].set(input);
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

    fn gui_options(&mut self, _: &mut egui::Ui, _: bool, _: DummyComponent, _: &[usize; 2], _: &mut EventWriter<UpdateComponentEvent>) {}
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
