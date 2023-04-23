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
pub struct Observer {
    id: String,
    #[reflect(ignore)]
    #[serde(skip)]
    ports: EnumMap<ObserverPorts, ComponentPortData>,
}

impl GridComponent for Observer {
    fn tick(&mut self, _: [usize; 2], tick: usize, world: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        let input = self.ports[ObserverPorts::Input].get();
        // Use SystemState instead of world.get_resource_mut() due to needing two mutable
        let mut system_state: SystemState<(ResMut<LevelData>, ResMut<SimIOPadded>)> = SystemState::new(world);
        let (mut level_data, mut sim_io) = system_state.get_mut(world);

        if input == 0 {
            sim_io.add_output(&mut level_data, tick, self.id.as_str(), None)
        } else {
            sim_io.add_output(&mut level_data, tick, self.id.as_str(), Some(input))
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

impl Observer {
    pub const CONST_PORTS: EnumMap<ObserverPorts, ([usize; 2], Side)> = EnumMap::from_array([([0, 0], Side::Left)]);
}

#[derive(Debug, Enum)]
pub enum ObserverPorts {
    Input,
}
