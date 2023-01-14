use bevy::prelude::*;

use super::shared::GridComponent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Wire {
    pub grid_pos: UVec2,
    pub connection_data: ConnectionData,
}

impl GridComponent for Wire {
    fn get_grid_pos(&self) -> UVec2 {
        self.grid_pos
    }
}
/// Struct of bools corrisponding to connections. UP DOWN LEFT RIGHT
/// Orginally used bitfields but moved away due to not needed
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct ConnectionData {
    pub blocked_sides: SideBlock,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub enum BlockState {
    Blocked,
    #[default]
    Normal,
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SideBlock {
    pub up: BlockState,
    pub down: BlockState,
    pub left: BlockState,
    pub right: BlockState,
}

impl ConnectionData {
    fn get_dir(&self, direction: &Direction) -> bool {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
    /// Direction should be given with respect to self
    pub fn check_link(&self, direction: &Direction, other_link: &ConnectionData) -> bool {
        self.get_dir(direction) && other_link.get_dir(&direction.reverse())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}