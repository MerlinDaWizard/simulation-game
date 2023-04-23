use crate::sim::{
    helpers::Side,
    model::{AudioEvent, GridComponent, SimulationData, VisualEvent, ComponentPortData, DummyComponent}, interactions::UpdateComponentEvent, levels::{SimIOPadded, LevelData}, self,
};
use bevy::{
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite}, prelude::{World, EventWriter, ResMut, System}, ecs::system::SystemState,
};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU8, Arc};

/// Observes the inputted value and records it to the data.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Provider {
    id: String,
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<ProviderPorts, ComponentPortData>,
}

impl GridComponent for Provider {
    fn tick(&mut self, _: [usize; 2], tick: usize, world: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input = self.ports[ProviderPorts::Output].get();
        // Use SystemState instead of world.get_resource_mut() due to needing two mutable
        let level_data = world.get_resource::<LevelData>().unwrap();
        let value = level_data.provided_inputs.get(self.id.as_str()).unwrap().get(tick);
        if let Some(num) = value {
            self.ports[ProviderPorts::Output].set(*num);
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

    fn gui_options(&mut self, _: &mut egui::Ui, _: bool, _: DummyComponent, _: &[usize; 2], _: &mut EventWriter<UpdateComponentEvent>) {}
}

impl Provider {
    pub const CONST_PORTS: EnumMap<ProviderPorts, ([usize; 2], Side)> = EnumMap::from_array([([0, 0], Side::Right)]);
}

#[derive(Debug, Enum)]
pub enum ProviderPorts {
    Output,
}
