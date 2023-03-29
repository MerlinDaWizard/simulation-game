use std::collections::HashSet;

use bevy::{reflect::{Reflect, FromReflect}, sprite::{TextureAtlasSprite, TextureAtlas}, prelude::Handle};
use itertools::izip;
use serde::{Deserialize, Serialize};
use crate::sim::model::{GridComponent, SimulationData, AudioEvent, VisualEvent, self, Direction, Port};
use strum::IntoEnumIterator;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Wire {
    links: [ConnectionStatus; 4],
    blocked_links: [SideState; 4],
}

impl GridComponent for Wire {
    // Wires do not need to tick as all communication is done intrinsically using the wire graph not graph
    fn tick(&mut self, own_pos: &(usize,usize), grid: &mut SimulationData) -> (Vec<VisualEvent>,Vec<AudioEvent>) {
        (Vec::new(),Vec::new())
    }

    fn build(&mut self, own_pos: &(usize,usize), sim_data: &mut SimulationData) {
        todo!()
    }

    fn on_place(&mut self, own_pos: &[usize; 2], sim_data: &mut SimulationData, sprite: &mut TextureAtlasSprite, atlas: &TextureAtlas) {
        self.links = [ConnectionStatus::Floating; 4];
        let mut ports = HashSet::new();
        
        for (i, d) in Direction::iter().enumerate() {
            if let SideState::Disabled = self.blocked_links[d.as_index()] {self.links[d.as_index()] = ConnectionStatus::Disabled; continue;} // Skip if side blocked
            let check_pos = combine_pos_offset(own_pos, &d.as_array()); // Might be an issue with L shaped and blocking sides
            if let Some(check_pos) = check_pos {
                let result = check_cell_unknown(&check_pos, sim_data);
                match result {
                    WireOrComponent::None => todo!(),
                    WireOrComponent::Wire => {self.links[d.as_index()] = ConnectionStatus::Connected},
                    WireOrComponent::Nodes(component_ports) => {
                        for p in component_ports {
                            ports.insert(p);
                        }
                    },
                }
            } else {continue;}
        }

        for (absolute_pos, direction) in ports.iter() {
            if absolute_pos == own_pos && self.blocked_links[direction.invert().as_index()] == SideState::Enabled {
                self.links[direction.invert().as_index()] = ConnectionStatus::Connected;
            }
        }
        let id = atlas.get_texture_index(&Handle::weak(self.links_to_sprite_name().into())).expect("No sprite for component on place");
        sprite.index = id;
    }

    /// Return the ports with the offset from the origin of the component.\
    /// E.g. ([0,0], Direction::Right)\
    /// for a port on the cell on the right
    fn ports(&self) -> Vec<([usize; 2], model::Direction)> {
        Vec::new()
    }

    fn ports_link(&mut self) -> Vec<([usize; 2], model::Direction, &mut model::Port)> {
        Vec::new()
    }
}

impl Wire {
    fn links_to_sprite_name(&self) -> String {
        let mut path = "wire_".to_string();
        let mut directions = Vec::with_capacity(4);
        for d in Direction::iter() {
            if self.links[d.as_index()] == ConnectionStatus::Connected {
                directions.push(d.as_str())
            }
        }

        path.push_str(&directions.join("_"));
        path
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Reflect, FromReflect, PartialEq, Eq)]
enum ConnectionStatus {
    Connected,
    #[default]
    Floating,
    /// Allow disabling of certain connections to allow wires running in parallel and such
    Disabled,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, Reflect, FromReflect, PartialEq, Eq)]
enum SideState {
    Disabled,
    #[default]
    Enabled,
}

fn combine_pos_offset(pos: &[usize; 2], offset: &[isize; 2]) -> Option<[usize; 2]> {
    let new_pos = [pos[0].checked_add_signed(offset[0]), pos[1].checked_add_signed(offset[1])];
    // I feel like there is a better way to do this doing more funcitonal style programming but oh well.
    let mut option_pos = [0usize; 2];
    for (i,x) in new_pos.iter().enumerate() {
        if let Some(v) = x {
            option_pos[i] = *v;
        } else {
            return None;
        }
    }
    return Some(option_pos);
}

fn check_cell_unknown(pos: &[usize; 2], sim_data: &mut SimulationData) -> WireOrComponent {
    if let Some(cell) = sim_data.grid.fetch(&pos) {
        match cell {
            model::CellState::Empty => {return WireOrComponent::None;},
            model::CellState::Reference(r) => {return check_cell_real(r, sim_data);},
            model::CellState::Real(c) => {return check_cell_real(pos, sim_data);},
        }
    } else {return WireOrComponent::None;}
}

fn check_cell_real(pos: &[usize; 2], sim_data: &SimulationData) -> WireOrComponent {
    let mut cell = sim_data.grid.fetch(&pos).expect("Attempted to fetch outside of grid afer reference");
    if let model::CellState::Real(c) = cell {
        if let model::Component::WirePiece(_) = c {return WireOrComponent::Wire;}
        let ports = c.ports();
        let mut absolute_pos_ports = Vec::with_capacity(ports.len());
        for (relative_pos, direction) in ports {
            absolute_pos_ports.push(([relative_pos[0]+pos[0],relative_pos[1]+pos[1]], direction))
        }
        return WireOrComponent::Nodes(c.ports());
    } else {panic!("Reference to non component")}
}


enum WireOrComponent {
    None,
    Wire,
    Nodes(Vec<([usize; 2], Direction)>),
}