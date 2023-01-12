use std::default;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

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
    mut commands: Commands,
    mut drag_start_events: EventReader<PointerDragStart>,
    mut drag_events: EventReader<PointerDrag>,
    mut drag_end_events: EventReader<PointerDragEnd>,

    pointers: Res<PointerMap>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
    locations: Query<&PointerLocation>,
    mut draggable_entity: Query<(Entity, &mut Sprite, &mut Draggable, &mut Transform, Option<&DragOpacity>, Option<&mut DragTypeReturn>)>,

) {
    for start in drag_start_events.iter() {
        let (_, mut sprite, mut draggable, transform, opacity, mut must_return) = match draggable_entity.get_mut(start.target()) {
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
        
        draggable.offset = transform.translation.truncate() - (pointer_position - (target_size / 2.0));
        if let Some(a) = opacity {
            sprite.color.set_a(a.0);
        }

        if let Some(mut r) = must_return {
            r.0 = transform.translation
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
        let (_, _, draggable, mut box_transform, _, _) = match draggable_entity.get_mut(dragging.target()) {
            Ok(e) => e,
            Err(_) => {
                continue;
            }
        };

        let z = box_transform.translation.z;
        box_transform.translation = (pointer_position - (target_size / 2.0) + draggable.offset).extend(z);
    }

    for end in drag_end_events.iter() {
        let (_, mut sprite, _, mut transform, opacity, must_return) = match draggable_entity.get_mut(end.target()) {
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
