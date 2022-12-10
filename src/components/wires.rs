use std::num::NonZeroU8;

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct GridPos(pub u8,pub u8);

/// Dummy wires
#[derive(Bundle)]
pub struct WireBundle {
    #[bundle]
    pub(crate) sprite: SpriteBundle,
    pub(crate) grid_pos: GridPos,
    pub(crate) connections: ConnectionData
}

/// Struct of bools corrisponding to connections. UP DOWN LEFT RIGHT
/// Orginally used bitfields but moved away due to not needed
#[derive(Component)]
pub struct ConnectionData {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
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
        return self.get_dir(direction) && other_link.get_dir(&direction.reverse());
    }
}

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



#[derive(Component)]
enum Components {
    Wires(WireBundle),
    // Other components eventually here aswell
}
