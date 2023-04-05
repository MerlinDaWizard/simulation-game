use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}, prelude::{Handle, error, debug}};
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};
use crate::sim::{model::{GridComponent, SimulationData, AudioEvent, VisualEvent, ComponentGrid, CellState, Component}, helpers::Side};
use crate::sim::helpers;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Wire {

}

impl GridComponent for Wire {
    // Wires do not need to tick as all communication is done intrinsically using the wire graph not graph
    fn tick(&mut self, own_pos: &[usize; 2], grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        (Vec::new(),Vec::new())
    }

    fn build(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&self, own_pos: &[usize; 2], sim_data: &SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        let mut sides = sim_data.port_grid.get_sides(own_pos);
        for (side, state) in sides.iter_mut() {
            let a = helpers::combine_offset(own_pos, &side.as_offset());
            if a.is_none() {continue;}
            debug!("{:?}", &a);
            debug!("{:?}",check_for_wire(&a.unwrap(), &sim_data.grid) );
            if check_for_wire(&a.unwrap(), &sim_data.grid) == true {
                *state = true;
            }
        }
        let sprite_name = sides_to_sprite_name(&sides);
        error!("{:?}",&sides);
        error!("{}",&sprite_name);
        let idx = atlas.get_texture_index(&Handle::weak(sprite_name.into())).expect("Could not find correct wire varient");
        sprite.index = idx;
    }

    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        Vec::new()
    }
}

impl Wire {
    pub const CONST_PORTS: EnumMap<WirePorts, ([usize; 2], Side)> = EnumMap::from_array([]);
}

/// Wire connections are a special system managed elsewhere
#[derive(Debug, Enum)]
pub enum WirePorts {}

enum ConnectionStatus {
    Connected,
    Floating,
    /// Allow disabling of certain connections to allow wires running in parallel and such
    Disabled,
}

/// I just did this so I could use ?, its kinda weird
fn check_for_wire_option(pos: &[usize; 2], grid: &ComponentGrid) -> Option<()> {
    let cell = grid.grid.get(pos[0])?.get(pos[1])?;
    if let CellState::Real(_, a) = cell {
        if let Component::WirePiece(_) = a {
            return Some(())
        }
    }
    None
}

fn check_for_wire(pos: &[usize; 2], grid: &ComponentGrid) -> bool {
    return check_for_wire_option(pos, grid).is_some()
}

fn sides_to_sprite_name(map: &EnumMap<Side, bool>) -> String {
    let mut path = "wire_".to_string();
    let mut sides = Vec::with_capacity(4);
    for (side, state) in map {
        if state == &true {
            sides.push(side.as_str())
        }
    }

    path.push_str(&sides.join("_"));
    path
}
