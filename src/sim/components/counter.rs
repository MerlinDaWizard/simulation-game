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

/// A counter which whenever it reads 100 on input A and 100 on input CLK (Clock) it will increment an internal counter and post that on the output
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Counter {
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<CounterPorts, ComponentPortData>,
    #[serde(skip)]
    count: u8, // Defaults 0
}

impl GridComponent for Counter {
    fn tick(&mut self, _: [usize; 2], _: usize, _: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input = self.ports[CounterPorts::Input].get();
        let input_clk = self.ports[CounterPorts::InputClk].get();
        if input_clk >= 255u8 && input >= 255u8 {
            self.count = self.count.wrapping_add(1);
            self.ports[CounterPorts::Output].set(self.count);
            // TODO: I would like to add an event to display the counter number ontop of the sprite.
        }
        (Vec::new(), Vec::new())
    }

    fn build(&mut self) {
        self.count = 0;
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
                self.ports[varient].set_link(Some(set_to));
                return Ok(());
            }
        }
        Err(())
    }

    fn show_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Test");
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
