use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_pixel_camera::PixelProjection;
use crate::game::{PlacementGrid, GRID_CELL_SIZE, GRID_CELL_AMOUNT_HEIGHT, GRID_CELL_AMOUNT_WIDTH};
use crate::components::shared::{Size, PlaceComponentEvent, GridPos};
use super::dummy_component::{GridLock, ComponentLink};

// Stores type and offset for use in dragging
// Must have this component to take effect
#[derive(Component, Default)]
pub struct Draggable {
    pub offset: Vec2,
}

/// Makes the entity return to starting position when dragged
/// Must be initialised
#[derive(Component)]
pub struct DragTypeReturn(pub Vec3);

impl DragTypeReturn {
    pub fn new() -> DragTypeReturn {
        DragTypeReturn(Vec3::ZERO)
    }
}

/// The opacity when dragging
#[derive(Component)]
pub struct DragOpacity(pub f32);

impl Draggable {
    pub fn new() -> Draggable {
        Draggable {
            offset: Vec2::ZERO,
        }
    }
}

pub fn drag_v2(
    _commands: Commands,
    mut drag_start_events: EventReader<PointerDragStart>,
    mut drag_events: EventReader<PointerDrag>,
    mut drag_end_events: EventReader<PointerDragEnd>,
    mut place_event_creator: EventWriter<PlaceComponentEvent>,

    pointers: Res<PointerMap>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    camera_query: Query<&PixelProjection, (With<Camera>, With<Camera2d>)>,
    locations: Query<&PointerLocation>,
    mut draggable_entity: Query<(Entity, &mut TextureAtlasSprite, &mut Draggable, &mut Transform, Option<&DragOpacity>, Option<&mut DragTypeReturn>, Option<&mut GridLock>, Option<&Size>, Option<&ComponentLink>), Without<PlacementGrid>>,
    placement_grid: Query<(&Sprite, &Transform, With<PlacementGrid>)>,
) {
    let grid = placement_grid.get_single().unwrap();
    //let bottom_left_corner = grid.1.translation.truncate() + Vec2::new(-112.0,-112.0);
    let grid_bottom_left_corner = grid.1.translation.truncate() + Vec2::new(-112.0,-112.0);
    let grid_top_right_corner = grid.1.translation.truncate() + Vec2::new(112.0, 112.0);

    let pixel_zoom = camera_query.single().zoom as f32;

    for start in drag_start_events.iter() {
        let (_, mut sprite, mut draggable, transform, opacity, must_return, gridlock, size, _) = match draggable_entity.get_mut(start.target()) {
            Ok(b) => b,
            Err(_)=> {
                continue;
            }
        };

        let pointer_entity = pointers.get_entity(start.pointer_id()).unwrap();
        let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
        let pointer_position = pointer_location.position;
        let target = pointer_location
            .target
            .get_render_target_info(&windows, &images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
        dbg!(target_size);
        let mouse_pos = (pointer_position - (target_size / 2.0)) / pixel_zoom;
        
        draggable.offset = mouse_pos - transform.translation.truncate();
        //draggable.offset = Vec2::ZERO;
        if let Some(a) = opacity {
            sprite.color.set_a(a.0);
        }

        if let Some(mut r) = must_return {
            r.0 = transform.translation
        }

        if let Some(mut gridlock) = gridlock {
            match size {
                Some(s) => {
                    let entity_bottom_left_corner = transform.translation.truncate();
                    let difference = mouse_pos - entity_bottom_left_corner;
                    gridlock.grab_part = (difference / GRID_CELL_SIZE).floor();
                },
                None => { // Assume it is the same size as the grid
                    // Keep gridlock grabpart as set by init (zero)
                }
            }
        }
    }

    for dragging in drag_events.iter() {
        let pointer_entity = pointers.get_entity(dragging.pointer_id()).unwrap();
        let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
        let pointer_position = pointer_location.position;
        let target = pointer_location
            .target
            .get_render_target_info(&windows, &images)
            .unwrap();
        let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
        let mouse_pos = (pointer_position - (target_size / 2.0)) / pixel_zoom;

        //dbg!(&boxes);
        //dbg!(&dragging.target());::new(DragType::Return(Vec2::ZERO))
        let (_, _, draggable, mut box_transform, _, _, gridlock, size, _) = match draggable_entity.get_mut(dragging.target()) {
            Ok(e) => e,
            Err(_) => {
                continue;
            }
        };

        let z = box_transform.translation.z;
        // dbg!(draggable.offset);
        
        if gridlock.is_some() && mouse_pos.x >= grid_bottom_left_corner.x && mouse_pos.y >= grid_bottom_left_corner.y && mouse_pos.x < grid_top_right_corner.x && mouse_pos.y < grid_top_right_corner.y {
            let gridlock = gridlock.unwrap(); // Weird way of doing it cuz I wanna put the above in one expression
            let grid_slot = ((mouse_pos - grid_bottom_left_corner) / GRID_CELL_SIZE).floor() - gridlock.grab_part;
            let size = size.unwrap();
            box_transform.translation = (grid_bottom_left_corner + Vec2::new(GRID_CELL_SIZE * grid_slot.x,GRID_CELL_SIZE * grid_slot.y)).extend(z);
        } else {
            box_transform.translation = (mouse_pos - draggable.offset).extend(z);
        }
    }

    for end in drag_end_events.iter() {
        let (_, mut sprite, _, mut transform, opacity, must_return, gridlock, _, places) = match draggable_entity.get_mut(end.target()) {
            Ok(b) => b,
            Err(_)=> {
                continue;
            }
        };
        if opacity.is_some() {
            sprite.color.set_a(1.0); // Might be better to store the orginal opacity and return to that instead of just always 1.0
        }

        if let Some(component) = places {
            match gridlock {
                None => {error!("Cannot place component without gridlock")},
                Some(g) => {
                    let pointer_entity = pointers.get_entity(end.pointer_id()).unwrap();
                    let pointer_location = locations.get(pointer_entity).unwrap().location().unwrap();
                    let pointer_position = pointer_location.position;
                    let target = pointer_location
                        .target
                        .get_render_target_info(&windows, &images)
                        .unwrap();
                    let target_size = target.physical_size.as_vec2() / target.scale_factor as f32;
                    let mouse_pos = (pointer_position - (target_size / 2.0)) / pixel_zoom;
                    let grid_slot = ((mouse_pos - grid_bottom_left_corner) / GRID_CELL_SIZE).floor() - g.grab_part;
                    if grid_slot.x >= 0.0 &&
                        grid_slot.y >= 0.0 &&
                        grid_slot.x < GRID_CELL_AMOUNT_WIDTH as f32 &&
                        grid_slot.y < GRID_CELL_AMOUNT_HEIGHT as f32
                    {
                        let grid_slot = grid_slot.as_uvec2();
                        place_event_creator.send(PlaceComponentEvent(GridPos(grid_slot), component.0.clone()));
                    }
                }
            }
        }
        if let Some(pos) = must_return {
            transform.translation = pos.0
        }
    }
}
