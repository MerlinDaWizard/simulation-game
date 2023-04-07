use bevy::prelude::{Vec2, UVec2};
use enum_map::Enum;
use strum_macros::EnumIter;

use crate::game::GRID_CELL_SIZE;

#[derive(Clone, Copy, Debug, EnumIter, Enum)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    pub fn as_offset(&self) -> [isize; 2] {
        match self {
            Side::Up => [0,1],
            Side::Down => [0,-1],
            Side::Left => [-1,0],
            Side::Right => [1,0],
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

    /// Rotates the side clockwise 90 deg.
    pub fn rotate(self) -> Self {
        match self {
            Side::Up => Side::Right,
            Side::Down => Side::Left,
            Side::Left => Side::Up,
            Side::Right => Side::Down,
        }
    }
}

pub fn combine_offset(pos: &[usize; 2], offset: &[isize; 2]) -> Option<[usize; 2]> {
    let new_pos = [pos[0].checked_add_signed(offset[0]), pos[1].checked_add_signed(offset[1])];
    // I feel like there is a better way to do this doing more funcitonal style programming but oh well.
    let mut option_pos = [0usize; 2];
    for (i,x) in new_pos.iter().enumerate() {
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
    let (mut x, mut y) = (0usize,0usize);
    let mut positions = Vec::with_capacity(size[0]*2 + size[1] * 2);

    for _i in 0..size[1] {
        y += 1;
        positions.push([x,y]);
    }
    y += 1;
    for _i in 0..size[0] {
        x += 1;
        positions.push([x,y]);
    }
    x += 1;
    for _i in 0..size[1] {
        y -= 1;
        positions.push([x,y]);
    }
    y -= 1;
    for _i in 0..size[0] {
        x -= 1;
        positions.push([x,y]);
    }
    //for abs_pos in positions.iter() {println!("[{},{}]",abs_pos[0],abs_pos[1]);}
    let abs_positions: Vec<[usize; 2]> = positions.iter_mut().filter_map(|relative_pos| {Some([(relative_pos[0]+pos[0]).checked_add_signed(-1)?,(relative_pos[1]+pos[1]).checked_add_signed(-1)?])}).collect();
    //println!("----------------");
    //for abs_pos in abs_positions.iter() {println!("[{},{}]",abs_pos[0],abs_pos[1]);}
    abs_positions
}

pub fn calc_grid_pos(grid_bottom_left: &Vec2, pos_in_grid: &UVec2) -> Vec2 {
    let pos = *grid_bottom_left + (pos_in_grid.as_vec2() * GRID_CELL_SIZE as f32);
    //dbg!(pos);
    pos
}