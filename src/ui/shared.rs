

use bevy::prelude::*;
use bevy::prelude::system_adapter::unwrap;
use bevy_mod_picking::prelude::*;

use crate::game::PlacementGrid;
use crate::components::shared::Size;
use super::dummy_component::GridLock;

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

    pointers: Res<PointerMap>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    locations: Query<&PointerLocation>,
    mut draggable_entity: Query<(Entity, &mut Sprite, &mut Draggable, &mut Transform, Option<&DragOpacity>, Option<&mut DragTypeReturn>, Option<&mut GridLock>, Option<&Size>), Without<PlacementGrid>>,
    placement_grid: Query<(&Sprite, &Transform, With<PlacementGrid>)>,
) {
    let grid = placement_grid.get_single().unwrap();
    //let bottom_left_corner = grid.1.translation.truncate() + Vec2::new(-112.0,-112.0);
    let grid_bottom_left_corner = grid.1.translation.truncate() + Vec2::new(-224.0,-224.0);
    let grid_top_right_corner = grid.1.translation.truncate() + Vec2::new(224.0, 224.0);

    for start in drag_start_events.iter() {
        let (_, mut sprite, mut draggable, transform, opacity, must_return, gridlock, size) = match draggable_entity.get_mut(start.target()) {
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
        let mouse_pos = pointer_position - (target_size / 2.0);
        
        draggable.offset = transform.translation.truncate() - (pointer_position - (target_size / 2.0));
        if let Some(a) = opacity {
            sprite.color.set_a(a.0);
        }

        if let Some(mut r) = must_return {
            r.0 = transform.translation
        }

        if let Some(mut gridlock) = gridlock {
            match size {
                Some(s) => {
                    let entity_bottom_left_corner = transform.translation.truncate() - s.0 * transform.scale.truncate() * 0.5;
                    let difference = mouse_pos - entity_bottom_left_corner;
                    dbg!(difference);
                    gridlock.grab_part = (difference / 64.0).floor();
                    dbg!(gridlock.grab_part);
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
        //dbg!(&boxes);
        //dbg!(&dragging.target());::new(DragType::Return(Vec2::ZERO))
        let (_, _, draggable, mut box_transform, _, _, gridlock, size) = match draggable_entity.get_mut(dragging.target()) {
            Ok(e) => e,
            Err(_) => {
                continue;
            }
        };

        let z = box_transform.translation.z;
        // dbg!(draggable.offset);
        
        let mouse_pos = pointer_position - (target_size / 2.0);
        if gridlock.is_some() && mouse_pos.x >= grid_bottom_left_corner.x && mouse_pos.y >= grid_bottom_left_corner.y && mouse_pos.x < grid_top_right_corner.x && mouse_pos.y < grid_top_right_corner.y {
            let gridlock = gridlock.unwrap(); // Weird way of doing it cuz I wanna put the above in one expression
            let grid_slot = ((mouse_pos - grid_bottom_left_corner) / 64.0).floor() - gridlock.grab_part;
            let size = size.unwrap();
            box_transform.translation = (grid_bottom_left_corner + Vec2::new(64.0*grid_slot.x,64.0*grid_slot.y) + (size.0 * box_transform.scale.truncate()) * 0.5).extend(z);
            // box_transform.translation = (bottom_left_corner).extend(z);
            // box_transform.translation = (bottom_left_corner + Vec2::new((mouse_pos.x / 64.0).floor() * 64.0,(mouse_pos.y / 64.0).floor() * 64.0) + (size.0 * box_transform.scale.truncate() / 2.0)).extend(z);
        } else {
            box_transform.translation = (pointer_position - (target_size / 2.0) + draggable.offset).extend(z);
        }
    }

    for end in drag_end_events.iter() {
        let (_, mut sprite, _, mut transform, opacity, must_return, _, _) = match draggable_entity.get_mut(end.target()) {
            Ok(b) => b,
            Err(_)=> {
                continue;
            }
        };
        if opacity.is_some() {
            sprite.color.set_a(1.0);
        }

        if let Some(pos) = must_return {
            transform.translation = pos.0
        }
    }
}
