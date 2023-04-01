use std::sync::{Arc, atomic::AtomicU8};
use bevy::reflect::{Reflect, FromReflect};
use enum_map::{EnumMap, Enum};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent}, port_grid::Side};

/// A 'And' gate component which should do typical AND behaviour, consider 100 ON, anything else OFF\
/// No connection defaults to 0 hence off
#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct GateAnd {
    #[reflect(ignore)] #[serde(skip)]
    ports: EnumMap<GateAndPorts, Option<Arc<AtomicU8>>>,
}

impl GridComponent for GateAnd {
    // And gate basic funtionality
    fn tick(&mut self, own_pos: &(usize, usize), grid: &mut SimulationData) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        // A && B = C
        // output = (100-input)
        todo!()
    }

    fn build(&mut self, own_pos: &(usize, usize), sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }
    
    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        return Self::CONST_PORTS.values().collect()
    }
}


impl GateAnd {
    const CONST_PORTS: EnumMap<GateAndPorts, ([usize; 2], Side)> = EnumMap::from_array([ ([0,1], Side::Left), ([0,1], Side::Right), ([0,0], Side::Right) ]);
}

#[derive(Debug, Enum)]
enum GateAndPorts {
    Input,
    OutputA,
    OutputB,
}