// Theres afew ways to store the ports, One way would be to have a 2d grid with each cell containing the top and left edges, these would be a  [Option<Port>; 2]
// Array of 2 due to having components feed directly into each other,
// this would make it fast and easy to build simulation however slightly annoying due to all the references and perhaps Mutex, Rwlock needed
// Alternatively whenever a wire updates we could iterate through each component then iterate through each port (implemented by trait)
// The trait impl would store the offset from bottom left (origin) of the component aswell as the direction it comes from
// We could then reverse all those directions then index the wire using these. Filter WirePos == PortDestinationPos
// Would require extra code for P2P (port to port) connections not through a wire
// Can already get a mental map for this

use std::sync::{Arc, atomic::{AtomicU8, Ordering}};

// Improvement: We add a new 'reference' component which just redirects any calls onto the actual cell the component is in.
// This means that instead of going through every component for ports we just go through the ones which are adjacent
use super::{
    helpers::{self, Side, spawn_component_sprite},
    port_grid::{{Port as PortGridPort}, PortGrid},
};
use crate::{
    components::placement::GridLink, sim::components::*, MainTextureAtlas,
};
use bevy::{prelude::*, sprite::Anchor};
use egui::Ui;
use enum_dispatch::enum_dispatch;
use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;

/// The type  that wires should store using

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct SimulationData {
    pub grid: ComponentGrid,
    #[reflect(ignore)]
    pub port_grid: PortGrid,
}

impl SimulationData {
    pub fn place_new_component(
        &mut self,
        commands: &mut Commands,
        grid_bottom_left: &Vec2,
        atlas: &TextureAtlas,
        main_atlas: &Res<MainTextureAtlas>,
        component_sprites: &mut Query<&mut TextureAtlasSprite, With<GridLink>>,
        dummy_component: DummyComponent,
        position: &[usize; 2],
    ) -> Result<(), PortGridError> {
        if self.grid.can_fit(position, &dummy_component.get_grid_size())
        {
            let mut sprite = TextureAtlasSprite::new(dummy_component.get_sprite_index(atlas));
            sprite.anchor = Anchor::BottomLeft;
            let mut component = dummy_component.build_default();
            component.on_place(position, self, &mut sprite, atlas);
            let entity_id = spawn_component_sprite(commands, sprite, grid_bottom_left, position, main_atlas.as_ref(), dummy_component);
            self.grid.place_component(entity_id, component, position);
            self.port_grid
                .modify_bulk(Some(PortGridPort::default()), dummy_component.ports(), position);
            let adjacent = helpers::get_adjacent(position, &dummy_component.get_grid_size());

            for component in adjacent {
                self.update_component(&component, component_sprites, atlas);
            }
        } else {
            return Err(PortGridError::CantFit);
        }

        Ok(())
    }

    /// When given a [Component] it should do all the port stuff but should not update surroundings\
    /// Doesnt bother checking can_fit
    pub fn load_component (
        &mut self,
        commands: &mut Commands,
         component: Component,
        grid_bottom_left: &Vec2,
        atlas: &TextureAtlas,
        main_atlas: &MainTextureAtlas,
        grid_position: &[usize; 2]
    ) {
        let dummy_component = component.dummy();
        let mut sprite = match &component {
            Component::WirePiece(w) => {
                let sprite_name = super::components::wire::sides_to_sprite_name(&w.connected_sides, "wire_", "_");
                let index = atlas.get_texture_index(&Handle::weak(sprite_name.into())).expect("Could not find correct wire varient");
                TextureAtlasSprite::new(index)
            }
            _ => TextureAtlasSprite::new(component.dummy().get_sprite_index(atlas))
        };
        sprite.anchor = Anchor::BottomLeft;
        let entity_id = spawn_component_sprite(commands, sprite, grid_bottom_left, grid_position, main_atlas, dummy_component);
        self.grid.place_component(entity_id, component, grid_position);
        self.port_grid.modify_bulk(Some(PortGridPort::default()), dummy_component.ports(), grid_position);
    }

    pub fn remove_component(
        &mut self,
        component: DummyComponent,
        position: [usize; 2],
    ) -> Result<(), PortGridError> {
        self.grid.remove_component(component, &position);
        self.port_grid
            .modify_bulk(None, component.ports(), &position);
        Ok(())
    }

    pub fn update_component(
        &mut self,
        position: &[usize; 2],
        component_sprites: &mut Query<&mut TextureAtlasSprite, With<GridLink>>,
        atlas: &TextureAtlas,
    ) -> Option<()> {
        let cell = self.grid.grid.get_mut(position[0])?.get_mut(position[1])?;
        //let cell = std::mem::replace(self.grid.grid.get_mut(position[0])?.get_mut(position[1])?, CellState::Empty);

        if let CellState::Real(id, comp) = cell {
            let mut sprite = component_sprites.get_mut(*id).unwrap();
            let mut current = std::mem::replace(comp, Component::SignalPassthrough(SignalPassthrough::default()));
            let comp = comp as *mut Component;
            current.on_place(position, self, sprite.as_mut(), atlas); // TODO:
            unsafe{
                let comp_mut: &mut Component = &mut *comp; // This is a HACKY solution and I MEAN HACKY
                *comp_mut = current;
            }
            //let cell = std::mem::replace(self.grid.grid.get_mut(position[0])?.get_mut(position[1])?, cell);
        }
        Some(())
    }
}

/// The 2d grid of components
#[derive(Debug, Default, Reflect, Serialize, Deserialize, Clone)]
pub struct ComponentGrid {
    pub grid: Vec<Vec<CellState>>,
}

pub enum PortGridError {
    CantFit,
}

impl ComponentGrid {
    pub fn can_fit(&self, position: &[usize; 2], size: &[usize; 2]) -> bool {
        for x in position[0]..(position[0] + size[0]) {
            let row = match self.grid.get(x) {
                Some(r) => r,
                None => {
                    return false;
                }
            };
            for y in position[1]..(position[1] + size[1]) {
                match row.get(y) {
                    None => {
                        return false;
                    }
                    Some(c) => match c {
                        CellState::Empty => {}
                        CellState::Reference(_) => return false,
                        CellState::Real(_, _) => return false,
                    },
                }
            }
        }
        true
    }

    /// Place a component in the grid, does not perform any overlap checks, these are done elsewhere. See [`Self::add_default_component()`]
    fn place_component(&mut self, entity_id: Entity, component: Component, position: &[usize; 2]) {
        let component_size = component.dummy().get_grid_size();
        let mut first = true; // Used to determin if to insert a real component or a grid reference
        for i in position[0]..(position[0] + component_size[0]) {
            for j in position[1]..(position[1] + component_size[1]) {
                match first {
                    true => {
                        first = false;
                        self.grid[i][j] = CellState::Real(entity_id, component.clone());
                    }
                    false => self.grid[i][j] = CellState::Reference(*position),
                }
            }
        }
    }

    fn remove_component(&mut self, component: DummyComponent, position: &[usize; 2]) {
        let component_size = component.get_grid_size();
        for i in position[0]..(position[0] + component_size[0]) {
            for j in position[1]..(position[1] + component_size[1]) {
                self.grid[i][j] = CellState::Empty;
            }
        }
    }
}
/// Contains Marker varients to pass around when wanting to create or refer to a type without all the data attached
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, EnumIter, Clone, Copy)]
pub enum DummyComponent {
    WirePiece,
    GateNot,
    GateAnd,
    SignalCopy,
    SignalPassthrough,
    Counter,
}

#[derive(Debug, Clone, Reflect, FromReflect, Serialize, Deserialize)]
pub enum CellState {
    /// An empty cell
    #[serde(rename = "E")]
    Empty,
    /// Stores the grid co-ordinates for master component
    #[serde(rename = "Ref")]
    Reference(#[serde(skip)] [usize; 2]),
    /// Contains a master component
    Real(#[serde(skip, default = "default_entity_fix")] Entity, Component),
}

fn default_entity_fix() -> Entity {
    Entity::PLACEHOLDER
}
/// Each possible component for a given cell\
/// If the cell is empty it should be expressed in the parent Option<> instead of here.
#[derive(Debug, Clone, Reflect, FromReflect, Serialize, Deserialize)]
#[enum_dispatch]
pub enum Component {
    WirePiece(Wire),
    GateNot(GateNot),
    GateAnd(GateAnd),
    SignalCopy(SignalCopy),
    SignalPassthrough(SignalPassthrough),
    Counter(Counter),
}

/// A struct to contain any (for player only) visual events to help with understanding whats happening\
/// E.g. [VisualEventType::Flash] to make a component light up
pub struct VisualEvent {
    pub placement: (f32, f32),
    pub event_type: VisualEventType,
}

/// The type of visual event to create.\
/// E.g. [VisualEventType::ElectricFizz] for a nice blue lightning particle effect
pub enum VisualEventType {
    ElectricFizz,
    Flash,
    Fire,
}

/// A struct to contain any audio events producted by the simulation\
/// E.g. Component Beeps
pub struct AudioEvent {
    pub path: &'static str,
    pub volume: f64,
}

/// The trait that every Component I use should implement to be usable in the simulation
#[enum_dispatch(Component)]
pub trait GridComponent {
    /// Whenever the sprite of the component should be updated
    fn on_place(
        &mut self,
        own_pos: &[usize; 2],
        sim_data: &SimulationData,
        sprite: &mut TextureAtlasSprite,
        atlas: &TextureAtlas,
    );
    /// When assembling the simulation + setting up, what should it reset / alter\
    fn build(&mut self);

    /// Should run the update on the component using itself
    fn tick(&mut self, own_pos: [usize; 2], tick_num: usize, world: &mut World) -> (Vec<VisualEvent>, Vec<AudioEvent>);

    /// Fetch a Vec of ports for use in the port grid
    fn ports(&self) -> Vec<&([usize; 2], Side)>;

    fn set_port(&mut self, offset: [usize; 2], side: Side, set_to: Arc<AtomicU8>) -> Result<(),() >;

    fn gui_options(&mut self, ui: &mut Ui, sim_halted: bool);
}


/// Each component which has ports should store an EnumMap<[ITS OWN PORTS], ComponentPortData>\
/// .get() to read\
/// .set() to write
#[derive(Default, Clone, Debug)]
pub struct ComponentPortData(Option<Arc<AtomicU8>>);

impl ComponentPortData {
    /// Read the value of a port, if no connection return the default value (0).
    pub fn get(&self) -> u8 {
        match &self.0 {
            None => 0,
            Some(p) => p.load(Ordering::Relaxed),
        }
    }

    /// Set the value of a port, if no connection ignore.
    pub fn set(&self, val: u8) {
        match &self.0 {
            None => {},
            Some(p) => {
                p.store(val, Ordering::Relaxed);
            }
        }
    }

    /// Take in an [Option<Arc<AtomicU8>>] and sets the internal state.
    pub fn set_link(&mut self, link: Option<Arc<AtomicU8>>) {
        self.0 = link;
    }
}