use bevy::prelude::*;
use bevy_mod_picking::prelude::ForwardedEvent;
use bevy_mod_picking::prelude::*;
use bevy_prototype_lyon::prelude::Path;
use bevy_prototype_lyon::prelude::ShapePath;
use bevy_prototype_lyon::shapes;
use bevy_prototype_lyon::shapes::RectangleOrigin;
use crate::MainTextureAtlas;
use crate::game::PlacementGridEntity;
use crate::GameState;
use crate::components::placement::GridLink;
use crate::components::placement::Size as SizeComponent;
use super::helpers;
use super::helpers::calc_grid_pos;
use super::model::{SimulationData, CellState};

pub struct GridComponentInteractionPlugin;

impl Plugin for GridComponentInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridComponentClick>()
        .add_event::<GridClick>()
        .add_event::<UpdateComponentEvent>()
        .init_resource::<SelectedComponent>()
        .add_systems((
            GridComponentClick::handle_events,
            show_activated_icon,
            grid_click_disable,
            update_component_listener,
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

#[derive(Debug, Clone)]
pub struct UpdateComponentEvent {
    pub pos: [usize; 2],
    pub surround: bool,
}

fn update_component_listener (
    mut listener: EventReader<UpdateComponentEvent>,
    mut sim_data: ResMut<SimulationData>,
    mut component_sprites: Query<&mut TextureAtlasSprite, With<GridLink>>,
    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
) {
    let atlas = atlases.get(&main_atlas.handle).unwrap();
    for event in listener.iter() {
        // Update itself.
        sim_data.update_component(&event.pos, &mut component_sprites, atlas);
        if event.surround { // Get positions of surroundings
            let adjacent = {
                let cell = &mut sim_data.grid.grid[event.pos[0]][event.pos[1]];
                if let CellState::Real(_, c) = cell {
                        helpers::get_adjacent(&event.pos, &c.dummy().get_grid_size())
                } else {error!("Attempted to update event on none real"); return}
            };
            // Update surroundings
            for component in adjacent {
                sim_data.update_component(&component, &mut component_sprites, atlas);
            }
        }
    }
}