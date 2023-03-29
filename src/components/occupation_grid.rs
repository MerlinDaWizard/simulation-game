use bevy::prelude::Resource;

use crate::game::GridSize;

#[derive(Resource, Default, Debug)]
pub struct OccupationGrid(pub Vec<Vec<CellInUse>>);

impl OccupationGrid {
    pub fn fill(&mut self, place_position: &[usize; 2], size: &[usize; 2], state: CellInUse) {
        for x in place_position[0]..place_position[0] + size[0] {
            let row = match self.0.get_mut(x) {
                Some(r) => r,
                None => {
                    continue;
                }
            };
            for y in place_position[1]..place_position[1] + size[1] {
                match row.get_mut(y) {
                    None => {continue;}
                    Some(c) => {
                        *c = state;
                    }
                }
            }
        }
    }

    pub fn can_fit(&self, place_position: &[usize; 2], size: &[usize; 2]) -> bool {
        dbg!(place_position);
        dbg!(size);
        for x in place_position[0]..(place_position[0] + size[0] - 1) {
            let row = match self.0.get(x) {
                Some(r) => r,
                None => {
                    return false;
                }
            };
            for y in place_position[1]..(place_position[1] + size[1] - 1) {
                match row.get(y) {
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

    pub fn empty_grid_from_size(size: &GridSize) -> OccupationGrid {
        OccupationGrid(vec![vec![CellInUse::Free; size.0[1]]; size.0[0]])
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum CellInUse {
    Occupied,
    #[default]
    Free,
}