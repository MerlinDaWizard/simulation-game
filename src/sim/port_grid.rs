use std::sync::{Arc, atomic::AtomicU8};

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
        PortGridData { left: LeftEdge::new(), top: TopEdge::new() }
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
        TopEdge { origin_up: None, origin_down: None }
    }
}

impl LeftEdge {
    pub fn new() -> Self {
        LeftEdge { origin_left: None, origin_right: None }
    }
}

#[derive(Debug, Clone)]
pub struct Port(Option<Arc<AtomicU8>>);

impl PortGrid {
    /// Size refers to the size of the corrisponding component grid.\
    /// The real size of the PortGrid will be +1 in each plane\
    /// Due to storing the top and left edges
    pub fn new_with_size(height: usize, width: usize) -> Self {
        PortGrid(vec![vec![PortGridData::new(); height+1]; width+1])
    }

    /// Position and side are with reference to the component grid.\
    /// Add a given port into a edge.\
    /// If already exists, will replace\
    /// Will error if out of size mapped
    pub fn insert(&mut self, position: &[usize; 2], side: Side, port: Option<Port>) -> Result<(), PortGridError> {
        match side {
            Side::Up => self.insert_top(position, Side::Down, port),
            Side::Down => self.insert_top(&[position[0],position[1]-1], Side::Up, port),
            Side::Left => self.insert_left(position, Side::Right, port),
            Side::Right => self.insert_left(&[position[0]+1, position[1]], Side::Left, port),
        }
    }

    /// internal insert used to do origins (two ports in a single edge)
    fn insert_top(&mut self, position: &[usize; 2], origin: Side, port: Option<Port>) -> Result<(), PortGridError> {
        let mut item = self.get_mut(position)?;
        match origin {
            Side::Up => {item.top.origin_up = port;},
            Side::Down => {item.top.origin_down = port;},
            _ => {return Err(PortGridError::IncorrectOrigin);},
        }
        Ok(())
    }

    /// internal insert used to do origins (two ports in a single edge)
    fn insert_left(&mut self, position: &[usize; 2], origin: Side, port: Option<Port>) -> Result<(), PortGridError> {
        let mut item = self.get_mut(position)?;
        match origin {
            Side::Left => {item.left.origin_left = port;},
            Side::Right => {item.left.origin_right = port;},
            _ => {return Err(PortGridError::IncorrectOrigin);},
        }
        Ok(())
    }

    /// Get a cell from the [PortGrid] accounting for any offsets as a result of the size+1 stuff
    fn get_mut(&mut self, position: &[usize; 2]) -> Result<&mut PortGridData, PortGridError> {
        let column = self.0.get_mut(position[0]).ok_or(PortGridError::PositionOutOfBounds)?;
        let mut item = column.get_mut(position[1] + 1).ok_or(PortGridError::PositionOutOfBounds)?;
        Ok(item)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

pub enum PortGridError {
    PositionOutOfBounds,
    IncorrectOrigin,
}