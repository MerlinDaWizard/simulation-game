use bevy::prelude::*;
use bevy_mod_picking::prelude::ForwardedEvent;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::Path;
use bevy_prototype_lyon::prelude::ShapePath;
use bevy_prototype_lyon::shapes;
use bevy_prototype_lyon::shapes::RectangleOrigin;
use crate::game::PlacementGridEntity;
use crate::GameState;
use crate::components::placement::GridLink;
use crate::components::placement::Size as SizeComponent;
use super::helpers::calc_grid_pos;
use super::model::{SimulationData, CellState};
use super::run::SimState;

pub struct GridComponentInteractionPlugin;

impl Plugin for GridComponentInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridComponentClick>()
        .add_event::<GridClick>()
        .init_resource::<SelectedComponent>()
        .add_systems((
            GridComponentClick::handle_events,
            show_activated_icon,
            grid_click_disable,
        ).distributive_run_if(in_state(GameState::InGame)));
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
        sim_state: Res<State<SimState>>,
    ) {
        for event in close.iter() {
            if sim_state.0 == SimState::Halted { // Only allow selection if Sim halted.
                if let Ok((_, link)) = grid_component.get(event.entity) {
                    selected_component.0 = Some(link.0);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub struct SelectedComponent(pub Option<[usize; 2]>);

fn show_activated_icon(
    sim_data: Res<SimulationData>,
    selected_component: ResMut<SelectedComponent>,
    placement_grid: Query<(&Sprite, &Transform, &SizeComponent), With<PlacementGridEntity>>,
    mut outline: Query<(&mut Transform, &mut Path, &mut Visibility), (With<SelectedComponentIndicator>, Without<PlacementGridEntity>)>,
) {
    if selected_component.is_changed() == false {return;} // Only run when there is a change on the selected component

    let grid = placement_grid.single();
    let grid_bottom_left = grid.1.translation.truncate() - (grid.2.0.as_vec2() * 0.5);

    if let Some(position) = selected_component.0 {
        let cell = &sim_data.grid.grid[position[0]][position[1]];
        if let CellState::Real(_, c) = cell {
            if let Ok(mut shape) = outline.get_single_mut() {
                let size = c.dummy().get_size();

                shape.0.translation = calc_grid_pos(&grid_bottom_left, &UVec2::new(position[0] as u32, position[1] as u32)).extend(shape.0.translation.z);
                *shape.2 = Visibility::Visible;

                let new_shape = shapes::Rectangle {
                    extents: Vec2::new(size[0], size[1]),
                    origin: RectangleOrigin::BottomLeft,
                };

                *shape.1 = ShapePath::build_as(&new_shape);
            }
        }
    } else { // Remove visibility of selection
        if let Ok(mut shape) = outline.get_single_mut() {
            *shape.2 = Visibility::Hidden;
        }
    }
}

#[derive(Clone, Copy, Debug, Component)]
pub struct SelectedComponentIndicator;

pub struct GridClick;

impl ForwardedEvent<PointerClick> for GridClick {
    fn from_data(_: &EventData<PointerClick>) -> Self {
        GridClick
    }
}

fn grid_click_disable(
    mut events: EventReader<GridClick>,
    mut selected_component: ResMut<SelectedComponent>,
) {
    for _ in events.iter() {
        selected_component.0 = None;
    }
}