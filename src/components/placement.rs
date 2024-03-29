use crate::game::{GridSize, PlacementGridEntity};
use crate::sim::components::*;
use crate::sim::helpers::Side;
use crate::sim::model::{
    CellState, Component as SimComponent, DummyComponent as DummySimComponent, SimulationData,
};
use crate::{GameState, MainTextureAtlas};
use bevy::prelude::*;
pub struct ComponentSetupPlugin;

impl Plugin for ComponentSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceComponentEvent>()
            .init_resource::<SimulationData>()
            .init_resource::<GridSize>()
            .register_type::<SimulationData>()
            .add_system(clear_grid.in_schedule(OnExit(GameState::InGame)))
            .add_system(placement_event.run_if(in_state(GameState::InGame)));
    }
}


fn clear_grid(mut simulation_grid: ResMut<SimulationData>) {
    for i in simulation_grid.grid.grid.iter_mut() {
        for mut _j in i.iter_mut() {
            _j = &mut CellState::Empty;
        }
    }
}

impl SimComponent {
    /// Converts it to it's dummy counterpart to get shared data
    pub fn dummy(&self) -> DummySimComponent {
        match self {
            SimComponent::WirePiece(_) => DummySimComponent::WirePiece,
            SimComponent::GateNot(_) => DummySimComponent::GateNot,
            SimComponent::GateAnd(_) => DummySimComponent::GateAnd,
            SimComponent::SignalCopy(_) => DummySimComponent::SignalCopy,
            SimComponent::SignalPassthrough(_) => DummySimComponent::SignalPassthrough,
            SimComponent::Counter(_) => DummySimComponent::Counter,
            SimComponent::Observer(_) => DummySimComponent::Observer,
            SimComponent::Provider(_) => DummySimComponent::Provider,
        }
    }
}

impl DummySimComponent {
    pub fn build_default(&self) -> SimComponent {
        match self {
            Self::WirePiece => SimComponent::WirePiece(Wire::default()),
            Self::GateNot => SimComponent::GateNot(GateNot::default()),
            Self::GateAnd => SimComponent::GateAnd(GateAnd::default()),
            Self::SignalCopy => SimComponent::SignalCopy(SignalCopy::default()),
            Self::SignalPassthrough => SimComponent::SignalPassthrough(SignalPassthrough::default()),
            Self::Counter => SimComponent::Counter(Counter::default()),
            Self::Observer => SimComponent::Observer(Observer::default()),
            Self::Provider => SimComponent::Provider(Provider::default()),
        }
    }

    pub fn get_sprite_index(&self, texture_atlas: &TextureAtlas) -> usize {
        let s = self.get_sprite_name();
        match texture_atlas.get_texture_index(&Handle::weak(s.into())) {
            Some(idx) => idx,
            None => {
                panic!("Attempted to load none existent texture {}", s);
            }
        }
    }

    pub fn get_sprite_name(&self) -> &str {
        let s = match self {
            Self::WirePiece => "wire_left_right",
            Self::GateNot => "gate_not",
            Self::GateAnd => "gate_and",
            Self::SignalCopy => "signal_copy",
            Self::SignalPassthrough => "signal_passthrough",
            Self::Counter => "signal_copy", // TODO: Make sprite for counter
            Self::Observer => "observer",
            Self::Provider => "provider",
        };
        s
    }

    pub fn get_size(&self) -> Vec2 {
        match self {
            Self::WirePiece => Vec2::splat(32.0),
            Self::GateNot => Vec2::splat(32.0),
            Self::GateAnd => Vec2::splat(64.0),
            Self::SignalCopy => Vec2::new(32.0, 64.0),
            Self::SignalPassthrough => Vec2::splat(32.0),
            Self::Counter => Vec2::new(32.0, 64.0),
            Self::Observer => Vec2::splat(32.0),
            Self::Provider => Vec2::splat(32.0),
        }
    }

    pub fn get_grid_size(&self) -> [usize; 2] {
        match self {
            Self::WirePiece => [1, 1],
            Self::GateNot => [1, 1],
            Self::GateAnd => [2, 2],
            Self::SignalCopy => [1, 2],
            Self::SignalPassthrough => [1, 1],
            Self::Counter => [1, 2],
            Self::Observer => [1, 1],
            Self::Provider => [1, 1],
        }
    }

    pub fn ports(&self) -> Vec<&([usize; 2], Side)> {
        match self {
            DummySimComponent::WirePiece => crate::sim::components::Wire::CONST_PORTS.values(),
            DummySimComponent::GateNot => crate::sim::components::GateNot::CONST_PORTS.values(),
            DummySimComponent::GateAnd => crate::sim::components::GateAnd::CONST_PORTS.values(),
            DummySimComponent::SignalCopy => crate::sim::components::SignalCopy::CONST_PORTS.values(),
            DummySimComponent::SignalPassthrough => crate::sim::components::SignalPassthrough::CONST_PORTS.values(),
            DummySimComponent::Counter => crate::sim::components::Counter::CONST_PORTS.values(),
            DummySimComponent::Observer => crate::sim::components::Observer::CONST_PORTS.values(),
            DummySimComponent::Provider => crate::sim::components::Provider::CONST_PORTS.values(),
        }.collect()
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            DummySimComponent::WirePiece => "Wire",
            DummySimComponent::GateNot => "NOT gate",
            DummySimComponent::GateAnd => "AND gate",
            DummySimComponent::SignalCopy => "Signal splitter",
            DummySimComponent::SignalPassthrough => "Signal passthrough",
            DummySimComponent::Counter => "Counter",
            DummySimComponent::Observer => "Observer",
            DummySimComponent::Provider => "Provider",
            
        }
    }

    pub fn desc(&self) -> &'static str {
        match self {
            DummySimComponent::WirePiece => "Wires transmit data instantly in an analogue manner.\nSides can be disabled in their menu",
            DummySimComponent::GateNot => "Inverts a given input.",
            DummySimComponent::GateAnd => "Checks if two signals are equal to 255, if so outputs 255.",
            DummySimComponent::SignalCopy => "Duplicates a signal acrosss two wires.",
            DummySimComponent::SignalPassthrough => "Passthoughs a signal, can be used for delays.",
            DummySimComponent::Counter => "Counts the amount of logic HIGHs on the input when there is also a HIGH on the clock signal.",
            DummySimComponent::Observer => "Watches the input and records the test data.",
            DummySimComponent::Provider => "Publishes test data into your circuit.",
        }
    }
}

#[derive(Debug, Component)]
pub struct Size(pub UVec2);

/// A component to link between sprites (entities) and the position in the [SimulationData] resource
#[derive(Debug, Component)]
pub struct GridLink(pub [usize; 2]);

pub struct PlaceComponentEvent(pub [usize; 2], pub DummySimComponent);

fn placement_event(
    mut commands: Commands,
    mut place_ev: EventReader<PlaceComponentEvent>,
    mut sim_data: ResMut<SimulationData>,
    placement_grid: Query<(&Sprite, &Transform, &Size), With<PlacementGridEntity>>,
    mut component_sprites: Query<&mut TextureAtlasSprite, With<GridLink>>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
) {
    let atlas = atlases.get(&main_atlas.handle).unwrap();
    let grid = placement_grid.single();
    let size = grid.2;
    let grid_bottom_left = grid.1.translation.truncate() - (size.0.as_vec2() * 0.5);
    for event in place_ev.iter() {
        match sim_data.place_new_component(
            &mut commands,
            &grid_bottom_left,
            atlas,
            &main_atlas,
            &mut component_sprites,
            event.1,
            &event.0,
        ) {
            Ok(_) => info!("Placed new component. {:?} at {:?}", event.1, event.0),
            Err(_) => info!("Attempted to place component in blocked position"),
        }
        //event.1.build_default().place()
    }
}
