use enum_map::{enum_map, EnumMap};
use std::sync::{atomic::AtomicU8, Arc};

use super::helpers::Side;

#[derive(Debug, Clone, Default)]
/// Each cell stores the Top and Left edge for its own grid\
/// \
/// Combining these we get a full graph of edges in a grid\
/// As a result of this the length needs to be + 1 for each direction\
/// and must have offsets applied to help it work
/// Refer to design:
pub struct PortGrid(Vec<Vec<PortGridData>>);

/// The data stored inside each real port grid cell.
#[derive(Debug, Clone)]
pub struct PortGridData {
    pub left: LeftEdge,
    pub top: TopEdge,
}

impl PortGridData {
    pub fn new() -> Self {
        PortGridData {
            left: LeftEdge::new(),
            top: TopEdge::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeftEdge {
    origin_left: Option<Port>,
    origin_right: Option<Port>,
}

#[derive(Debug, Clone)]
pub struct TopEdge {
    origin_up: Option<Port>,
    origin_down: Option<Port>,
}

impl TopEdge {
    pub fn new() -> Self {
        TopEdge {
            origin_up: None,
            origin_down: None,
        }
    }
}

impl LeftEdge {
    pub fn new() -> Self {
        LeftEdge {
            origin_left: None,
            origin_right: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Port(pub Option<Arc<AtomicU8>>);

impl PortGrid {
    /// Size refers to the size of the corrisponding component grid.\
    /// The real size of the PortGrid will be +1 in each plane\
    /// Due to storing the top and left edges
    pub fn new_with_size(height: usize, width: usize) -> Self {
        PortGrid(vec![vec![PortGridData::new(); height + 1]; width + 1])
    }

    /// Position and side are with reference to the component grid.\
    /// Add a given port into a edge.\
    /// If already exists, will replace\
    /// Will error if out of size mapped
    pub fn insert(
        &mut self,
        position: &[usize; 2],
        side: Side,
        port: Option<Port>,
    ) -> Result<(), PortGridError> {
        match side {
            Side::Up => self.insert_top(position, Side::Down, port),
            Side::Down => self.insert_top(&[position[0], position[1] - 1], Side::Up, port),
            Side::Left => self.insert_left(position, Side::Right, port),
            Side::Right => self.insert_left(&[position[0] + 1, position[1]], Side::Left, port),
        }
    }

    /// internal insert used to do origins (two ports in a single edge)
    fn insert_top(
        &mut self,
        position: &[usize; 2],
        origin: Side,
        port: Option<Port>,
    ) -> Result<(), PortGridError> {
        let mut item = self.get_mut(position)?;
        match origin {
            Side::Up => {
                item.top.origin_up = port;
            }
            Side::Down => {
                item.top.origin_down = port;
            }
            _ => {
                return Err(PortGridError::IncorrectOrigin);
            }
        }
        Ok(())
    }

    /// internal insert used to do origins (two ports in a single edge)
    fn insert_left(
        &mut self,
        position: &[usize; 2],
        origin: Side,
        port: Option<Port>,
    ) -> Result<(), PortGridError> {
        let mut item = self.get_mut(position)?;
        match origin {
            Side::Left => {
                item.left.origin_left = port;
            }
            Side::Right => {
                item.left.origin_right = port;
            }
            _ => {
                return Err(PortGridError::IncorrectOrigin);
            }
        }
        Ok(())
    }

    /// Get a cell from the [PortGrid] accounting for any offsets as a result of the size+1 stuff
    fn get_mut(&mut self, position: &[usize; 2]) -> Result<&mut PortGridData, PortGridError> {
        let column = self
            .0
            .get_mut(position[0])
            .ok_or(PortGridError::PositionOutOfBounds)?;
        let item = column
            .get_mut(position[1] + 1)
            .ok_or(PortGridError::PositionOutOfBounds)?;
        Ok(item)
    }

    /// Clones target inserting it into the correct edge
    pub fn modify_bulk(
        &mut self,
        target: Option<Port>,
        list: Vec<&([usize; 2], Side)>,
        offset: &[usize; 2],
    ) {
        for (pos, side) in list {
            //dbg!(pos);
            //dbg!([pos[0]+offset[0], pos[1]+offset[1]]);
            //dbg!(side);
            self.insert(
                &[pos[0] + offset[0], pos[1] + offset[1]],
                *side,
                target.clone(),
            )
            .expect("Attempted to execute modify outside of grid");
        }
    }

    pub fn get_sides(&self, position: &[usize; 2]) -> EnumMap<Side, bool> {
        enum_map! {
            Side::Up => self.get_side(position, Side::Up).unwrap_or(false),
            Side::Down => self.get_side(position, Side::Down).unwrap_or(false),
            Side::Left => self.get_side(position, Side::Left).unwrap_or(false),
            Side::Right => self.get_side(position, Side::Right).unwrap_or(false),
        }
    }

    /// Get side based of component grid position\
    /// Ignores ports from its own position
    pub fn get_side(&self, position: &[usize; 2], side: Side) -> Result<bool, PortGridError> {
        match side {
            Side::Up => Ok(self.get(position)?.top.origin_up.is_some()),
            Side::Down => Ok(self
                .get(&[
                    position[0],
                    position[1]
                        .checked_sub(1)
                        .ok_or(PortGridError::PositionOutOfBounds)?,
                ])?
                .top
                .origin_down
                .is_some()),
            Side::Left => Ok(self.get(position)?.left.origin_left.is_some()),
            Side::Right => Ok(self
                .get(&[position[0] + 1, position[1]])?
                .left
                .origin_right
                .is_some()),
        }
    }

    /// Get a cell from the [PortGrid] accounting for any offsets as a result of the size+1 stuff
    pub fn get(&self, position: &[usize; 2]) -> Result<&PortGridData, PortGridError> {
        let column = self
            .0
            .get(position[0])
            .ok_or(PortGridError::PositionOutOfBounds)?;
        let item = column
            .get(position[1] + 1)
            .ok_or(PortGridError::PositionOutOfBounds)?;
        Ok(item)
    }
}

#[derive(Debug)]
pub enum PortGridError {
    PositionOutOfBounds,
    IncorrectOrigin,
}
