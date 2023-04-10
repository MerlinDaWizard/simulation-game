//! A raycasting backend for [`bevy_sprite`](bevy::sprite).

#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![deny(missing_docs)]

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_mod_picking::core::backend::prelude::*;
use bevy_pixel_camera::PixelProjection;

/// Adds improved picking support for [`bevy_sprite`](bevy::sprite)
#[derive(Clone)]
pub struct MerlinSpriteBackend;
impl PickingBackend for MerlinSpriteBackend {}
impl Plugin for MerlinSpriteBackend {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "sprite")]
        app.add_system(sprite_picking.in_set(PickSet::Backend));
        #[cfg(feature = "texture_atlas_sprite")]
        app.add_system(texture_atlas_sprite_picking.in_set(PickSet::Backend));
    }
}

/// Checks if any sprite entities are under each pointer
#[cfg(feature = "sprite")]
pub fn sprite_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    windows: Query<(Entity, &Window)>,
    images: Res<Assets<Image>>,
    #[cfg(feature = "pixel_camera")]
    camera_query: Query<&PixelProjection, (With<Camera>, With<Camera2d>)>,
    sprite_query: Query<(
        Entity,
        &Sprite,
        &Handle<Image>,
        &GlobalTransform,
        &ComputedVisibility,
        Option<&FocusPolicy>,
    )>,
    mut output: EventWriter<EntitiesUnderPointer>,
) {
    let mut sorted_sprites: Vec<_> = sprite_query.iter().collect();

    sorted_sprites.sort_by(|a, b| {
        (b.3.translation().z)
            .partial_cmp(&a.3.translation().z)
            .unwrap_or(Ordering::Equal)
    });

    #[cfg(feature = "pixel_camera")]
    let pixel_zoom = camera_query.single().zoom;
    for (pointer, location) in pointers.iter().filter_map(|(pointer, pointer_location)| {
        pointer_location.location().map(|loc| (pointer, loc))
    }) {
        let cursor_position = location.position;
        let mut blocked = false;

        let over_list = sorted_sprites
            .iter()
            .copied()
            .filter_map(
                |(entity, sprite, image, global_transform, visibility, focus)| {
                    if blocked || !visibility.is_visible() {
                        return None;
                    }

                    let position = global_transform.translation();
                    let sprite_position = position.truncate();

                    let extents = sprite
                        .custom_size
                        .or_else(|| images.get(image).map(|f| f.size()))
                        .map(|size| size / 2.0 * global_transform.to_scale_rotation_translation().0.truncate())?;

                    let anchor_offset = sprite.anchor.as_vec() * extents * 2.0;

                    let target = if let Some(t) =
                        location.target.get_render_target_info(&windows, &images)
                    {
                        t.physical_size.as_vec2() / t.scale_factor as f32
                    } else {
                        return None;
                    };

                    #[cfg(not(feature = "pixel_camera"))]
                    let min = sprite_position - extents + anchor_offset + target / 2.0;
                    #[cfg(not(feature = "pixel_camera"))]
                    let max = sprite_position + extents + anchor_offset + target / 2.0;

                    #[cfg(feature = "pixel_camera")]
                    let min = pixel_zoom as f32 * (sprite_position - extents + anchor_offset) + target / 2.0;
                    #[cfg(feature = "pixel_camera")]
                    let max = pixel_zoom as f32 * (sprite_position + extents + anchor_offset) + target / 2.0;

                    let contains_cursor = (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y);

                    blocked = focus != Some(&FocusPolicy::Pass) && contains_cursor;

                    contains_cursor.then_some(EntityDepth {
                        entity,
                        depth: -position.z,
                    })
                },
            )
            .collect::<Vec<_>>();

        output.send(EntitiesUnderPointer {
            pointer: *pointer,
            over_list,
        })
    }
}

/// Checks if any sprite entities are under each pointer
#[cfg(feature = "texture_atlas_sprite")]
pub fn texture_atlas_sprite_picking(
    pointers: Query<(&PointerId, &PointerLocation)>,
    windows: Query<(Entity, &Window)>,
    images: Res<Assets<Image>>,
    #[cfg(feature = "pixel_camera")]
    camera_query: Query<&PixelProjection, (With<Camera>, With<Camera2d>)>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    sprite_query: Query<(
        Entity,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &GlobalTransform,
        &ComputedVisibility,
        Option<&FocusPolicy>,
    )>,
    mut output: EventWriter<EntitiesUnderPointer>,
) {
    let mut sorted_sprites: Vec<_> = sprite_query.iter().collect();

    sorted_sprites.sort_by(|a, b| {
        (b.3.translation().z)
            .partial_cmp(&a.3.translation().z)
            .unwrap_or(Ordering::Equal)
    });

    #[cfg(feature = "pixel_camera")]
    let pixel_zoom = camera_query.single().zoom;
    for (pointer, location) in pointers.iter().filter_map(|(pointer, pointer_location)| {
        pointer_location.location().map(|loc| (pointer, loc))
    }) {
        let cursor_position = location.position;
        let mut blocked = false;

        let over_list = sorted_sprites
            .iter()
            .copied()
            .filter_map(
                |(entity, sprite, atlas_handle, global_transform, visibility, focus)| {
                    if blocked || !visibility.is_visible() {
                        return None;
                    }

                    let position = global_transform.translation();
                    let sprite_position = position.truncate();

                    let extents = sprite
                        .custom_size
                        .or_else(|| texture_atlases.get(atlas_handle).map(|a| a.textures[sprite.index].size()))
                        .map(|size| size / 2.0 * global_transform.to_scale_rotation_translation().0.truncate())?;

                    let anchor_offset = sprite.anchor.as_vec() * extents * -2.0;

                    let target = if let Some(t) =
                        location.target.get_render_target_info(&windows, &images)
                    {
                        t.physical_size.as_vec2() / t.scale_factor as f32
                    } else {
                        return None;
                    };

                    #[cfg(not(feature = "pixel_camera"))]
                    let min = sprite_position - extents + anchor_offset + target / 2.0;
                    #[cfg(not(feature = "pixel_camera"))]
                    let max = sprite_position + extents + anchor_offset + target / 2.0;

                    #[cfg(feature = "pixel_camera")]
                    let min = pixel_zoom as f32 * (sprite_position - extents + anchor_offset) + target / 2.0;
                    #[cfg(feature = "pixel_camera")]
                    let max = pixel_zoom as f32 * (sprite_position + extents + anchor_offset) + target / 2.0;

                    let contains_cursor = (min.x..max.x).contains(&cursor_position.x)
                        && (min.y..max.y).contains(&cursor_position.y);

                    blocked = focus != Some(&FocusPolicy::Pass) && contains_cursor;

                    contains_cursor.then_some(EntityDepth {
                        entity,
                        depth: -position.z,
                    })
                },
            )
            .collect::<Vec<_>>();

        output.send(EntitiesUnderPointer {
            pointer: *pointer,
            over_list,
        })
    }
}
