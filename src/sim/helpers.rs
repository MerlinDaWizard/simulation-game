use bevy::{prelude::{UVec2, Vec2, Commands, Entity, Transform, Name}, reflect::{FromReflect, Reflect}, sprite::{SpriteSheetBundle, TextureAtlasSprite}};
use enum_map::Enum;
use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;

use crate::{game::{GRID_CELL_SIZE, GameRoot}, components::placement::GridLink, MainTextureAtlas};

use super::model::{Component, DummyComponent};

#[derive(Clone, Copy, Debug, EnumIter, Enum, PartialEq, Eq, Reflect, FromReflect, Serialize, Deserialize)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    pub fn as_offset(&self) -> [isize; 2] {
        match self {
            Side::Up => [0, 1],
            Side::Down => [0, -1],
            Side::Left => [-1, 0],
            Side::Right => [1, 0],
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Side::Up => "up",
            Side::Down => "down",
            Side::Left => "left",
            Side::Right => "right",
        }
    }

    pub fn reverse(self) -> Self {
        match self {
            Side::Up => Side::Down,
            Side::Down => Side::Up,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

pub fn combine_offset(pos: &[usize; 2], offset: &[isize; 2]) -> Option<[usize; 2]> {
    let new_pos = [
        pos[0].checked_add_signed(offset[0]),
        pos[1].checked_add_signed(offset[1]),
    ];
    // I feel like there is a better way to do this doing more funcitonal style programming but oh well.
    let mut option_pos = [0usize; 2];
    for (i, x) in new_pos.iter().enumerate() {
        if let Some(v) = x {
            option_pos[i] = *v;
        } else {
            return None;
        }
    }
    Some(option_pos)
}

/// A hacky method to get the adjacent squares of a variable sized component at a variable position.\
/// WARNING: DOES NOT CHECK ABOVE GRID SIZE ONLY BELOW
pub fn get_adjacent(pos: &[usize; 2], size: &[usize; 2]) -> Vec<[usize; 2]> {
    let (mut x, mut y) = (0usize, 0usize);
    let mut positions = Vec::with_capacity(size[0] * 2 + size[1] * 2);

    for _i in 0..size[1] {
        y += 1;
        positions.push([x, y]);
    }
    y += 1;
    for _i in 0..size[0] {
        x += 1;
        positions.push([x, y]);
    }
    x += 1;
    for _i in 0..size[1] {
        y -= 1;
        positions.push([x, y]);
    }
    y -= 1;
    for _i in 0..size[0] {
        x -= 1;
        positions.push([x, y]);
    }
    //for abs_pos in positions.iter() {println!("[{},{}]",abs_pos[0],abs_pos[1]);}
    let abs_positions: Vec<[usize; 2]> = positions
        .iter_mut()
        .filter_map(|relative_pos| {
            Some([
                (relative_pos[0] + pos[0]).checked_add_signed(-1)?,
                (relative_pos[1] + pos[1]).checked_add_signed(-1)?,
            ])
        })
        .collect();
    //println!("----------------");
    //for abs_pos in abs_positions.iter() {println!("[{},{}]",abs_pos[0],abs_pos[1]);}
    abs_positions
}

pub fn calc_grid_pos(grid_bottom_left: &Vec2, pos_in_grid: &UVec2) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.as_vec2() * GRID_CELL_SIZE as f32);
    //dbg!(pos);
    pos
}

pub fn spawn_component_sprite(commands: &mut Commands, sprite: TextureAtlasSprite, grid_bottom_left: &Vec2, position: &[usize; 2], main_atlas: &MainTextureAtlas, dummy_component: DummyComponent) -> Entity {
    commands
        .spawn((
            SpriteSheetBundle {
                sprite,
                transform: Transform {
                    translation: calc_grid_pos(
                        grid_bottom_left,
                        &UVec2::new(position[0] as u32, position[1] as u32),
                    )
                    .extend(11.0),
                    ..Default::default()
                },
                texture_atlas: main_atlas.handle.clone(),
                ..Default::default()
            },
            GameRoot,
            GridLink(*position),
            Name::new(format!(
                "Component - {}",
                dummy_component.get_sprite_name()
            )),
        ))
        .id()
}