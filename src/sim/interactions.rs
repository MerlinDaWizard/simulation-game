use bevy::prelude::*;
use bevy_mod_picking::prelude::ForwardedEvent;
use bevy_mod_picking::prelude::*;

use crate::GameState;
use crate::components::placement::GridLink;

pub struct GridComponentInteractionPlugin;

impl Plugin for GridComponentInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridComponentClick>()
        .init_resource::<SelectedComponent>()
        .add_system(GridComponentClick::handle_events.run_if(in_state(GameState::InGame)));
    }
}

pub struct GridComponentClick {
    entity: Entity,
}

impl ForwardedEvent<PointerClick> for GridComponentClick {
    fn from_data(event_data: &EventData<PointerClick>) -> Self {
        GridComponentClick { entity: event_data.target()}
    }
}

impl GridComponentClick {
    fn handle_events(
        _commands: Commands,
        mut close: EventReader<GridComponentClick>,
        grid_component: Query<(Entity, &GridLink)>,
        mut selected_component: ResMut<SelectedComponent>,
    ) {
        for event in close.iter() {
            if let Ok((_, link)) = grid_component.get(event.entity) {
                selected_component.0 = Some(link.0);
            }
        }
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct SelectedComponent(pub Option<[usize; 2]>);