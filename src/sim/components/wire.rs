use std::sync::Arc;
use std::sync::atomic::AtomicU8;

use crate::sim::helpers;
use crate::sim::{
    helpers::Side,
    model::{
        AudioEvent, CellState, Component, ComponentGrid, GridComponent, SimulationData, VisualEvent,
    },
};
use bevy::prelude::World;
use bevy::{
    prelude::{debug, Handle},
    reflect::{FromReflect, Reflect},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use egui::{TextFormat, RichText};
use egui::text::LayoutJob;
use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};


#[derive(Debug, Default, Serialize, Deserialize, Clone, Reflect, FromReflect)]
pub struct Wire {
    #[reflect(ignore)]
    pub connected_sides: EnumMap<helpers::Side, bool>,
    #[reflect(ignore)]
    pub disabled_sides: EnumMap<helpers::Side, EnabledOrDisabled>
}

impl GridComponent for Wire {
    // Wires do not need to tick as all communication is done intrinsically using the wire graph not graph
    fn tick(&mut self, _: [usize; 2], _: usize, _: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>) {
        (Vec::new(), Vec::new())
    }

    fn build(&mut self) {}

    fn on_place(
        &mut self,
        own_pos: &[usize; 2],
        sim_data: &SimulationData,
        sprite: &mut TextureAtlasSprite,
        atlas: &TextureAtlas,
    ) {
        //dbg!(own_pos);
        let mut sides = sim_data.port_grid.get_sides(own_pos);
        for (side, state) in sides.iter_mut() {
            if self.disabled_sides[side] == EnabledOrDisabled::Disabled {*state = false; continue;}
            let a = helpers::combine_offset(own_pos, &side.as_offset());
            if a.is_none() {
                continue;
            }
            debug!("{:?}", &a);
            debug!("{:?}", check_for_wire_link(&a.unwrap(), &sim_data.grid, side.reverse()));
            if check_for_wire_link(&a.unwrap(), &sim_data.grid, side.reverse()) {
                *state = true;
            }
        }
        let sprite_name = sides_to_sprite_name(&sides, "wire_", "_");
        //debug!("{:?}",&sides);
        //debug!("{}",&sprite_name);
        let idx = atlas
            .get_texture_index(&Handle::weak(sprite_name.into()))
            .expect("Could not find correct wire varient");
        sprite.index = idx;
        self.connected_sides = sides;
    }

    fn ports(&self) -> Vec<&([usize; 2], Side)> {
        Vec::new()
    }

    fn set_port(&mut self, _: [usize; 2], _: Side, _: Arc<AtomicU8>) -> Result<(),()> {
        Err(())
    }

    fn gui_options(&mut self, ui: &mut egui::Ui) {
        let mut connected_sides = sides_to_sprite_name(&self.connected_sides, "", ", ");
        if connected_sides.is_empty() {
            connected_sides = String::from("None");
        }
        ui.horizontal(|ui| {
            ui.label("Connected sides: ");
            ui.label(RichText::new(connected_sides).code());
        });
        ui.heading("Enabled sides:");
        for (side, state) in &mut self.disabled_sides {
            if ui.checkbox(&mut (*state == EnabledOrDisabled::Enabled), side.as_str()).changed() {
                *state = match *state {
                    EnabledOrDisabled::Disabled => EnabledOrDisabled::Enabled,
                    EnabledOrDisabled::Enabled => EnabledOrDisabled::Disabled,
                };
            }
        }
    }
}

impl Wire {
    pub const CONST_PORTS: EnumMap<WirePorts, ([usize; 2], Side)> = EnumMap::from_array([]);
}

/// Wire connections are a special system managed elsewhere
#[derive(Debug, Enum)]
pub enum WirePorts {}


#[derive(Clone, Copy, Reflect, FromReflect, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnabledOrDisabled {
    #[default]
    Enabled,
    Disabled,
}

/// I just did this so I could use ?, its kinda weird
fn check_for_wire_option(pos: &[usize; 2], grid: &ComponentGrid, origin_side: Side) -> Option<()> {
    let cell = grid.grid.get(pos[0])?.get(pos[1])?;
    if let CellState::Real(_, a) = cell {
        if let Component::WirePiece(w) = a { // Check the side is enabled to prevent wire connection missmatch
            if w.disabled_sides[origin_side] == EnabledOrDisabled::Enabled {
                return Some(());
            }
        }
    }
    None
}

/// Check for a wire link at this position.\
/// Origin side is relative to the checked position.
fn check_for_wire_link(pos: &[usize; 2], grid: &ComponentGrid, origin_side: Side) -> bool {
    check_for_wire_option(pos, grid, origin_side).is_some()
}

pub fn sides_to_sprite_name(map: &EnumMap<Side, bool>, starter: &str, seperator: &str) -> String {
    let mut path = starter.to_string();
    let mut sides = Vec::with_capacity(4);
    for (side, state) in map {
        if *state == true {
            sides.push(side.as_str())
        }
    }

    path.push_str(&sides.join(seperator));
    path
}
