use bevy::prelude::{UVec2, Resource};
use crate::game::{GRID_CELL_AMOUNT_HEIGHT, GRID_CELL_AMOUNT_WIDTH};

use super::shared::GridPos;

#[derive(Debug, Resource, Default)]
pub struct Grid {
    occupied_cells: [[CellInUse; GRID_CELL_AMOUNT_WIDTH as usize]; GRID_CELL_AMOUNT_HEIGHT as usize],

}

impl Grid {
    pub fn fill(&mut self, place_position: &GridPos, size: &UVec2, state: CellInUse) {
        for i in place_position.0.x..place_position.0.x + size.x {
            let row = match self.occupied_cells.get_mut(i as usize) {
                Some(r) => r,
                None => {
                    continue;
                }
            };
            for j in place_position.0.y..place_position.0.y+size.y {
                match row.get_mut(j as usize) {
                    None => {continue;}
                    Some(c) => {
                        *c = state;
                    }
                }
            }
        }
    }

    pub fn can_fit(&self, place_position: &GridPos, size: &UVec2) -> bool {
        for i in place_position.0.x..place_position.0.x + size.x {
            let row = match self.occupied_cells.get(i as usize) {
                Some(r) => r,
                None => {
                    return false;
                }
            };
            for j in place_position.0.y..place_position.0.y+size.y {
                match row.get(j as usize) {
                    None => {return false;}
                    Some(c) => {
                        if c == &CellInUse::Occupied {
                            return false;
                        }
                    }
                }
            }
        }
        return true;
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum CellInUse {
    Occupied,
    #[default]
    Free,
}