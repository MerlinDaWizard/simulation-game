use enum_map::{enum_map, EnumMap};
use serde::{Serialize, Deserialize};
use super::helpers::Side;

#[derive(Debug, Clone, Default)]
/// Each cell stores the Top and Left edge for its own grid\
/// \
/// Combining these we get a full graph of edges in a grid\
/// As a result of this the length needs to be + 1 for each direction\
/// and must have offsets applied to help it work
/// Refer to design:
pub struct PortGrid(pub Vec<Vec<PortGridData>>);

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

    pub fn reset_build(&mut self) {
        if let Some(p) = &mut self.origin_up {
            p.mark_checked(false);
        }

        if let Some(p) = &mut self.origin_down {
            p.mark_checked(false);
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

    pub fn reset_build(&mut self) {
        if let Some(p) = &mut self.origin_left {
            p.mark_checked(false);
        }

        if let Some(p) = &mut self.origin_right {
            p.mark_checked(false);
        }
    }
}


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
            Side::Up => self.get_side_existance(position, Side::Up).unwrap_or(false),
            Side::Down => self.get_side_existance(position, Side::Down).unwrap_or(false),
            Side::Left => self.get_side_existance(position, Side::Left).unwrap_or(false),
            Side::Right => self.get_side_existance(position, Side::Right).unwrap_or(false),
        }
    }

    /// Get side based of component grid position\
    /// Ignores ports from its own position
    pub fn get_side_existance(&self, position: &[usize; 2], side: Side) -> Result<bool, PortGridError> {
        self.get_port(position, side).and_then(|a| Ok(a.is_some()))
    }

    /// Get side based of component grid position\
    /// Ignores ports from its own position
    pub fn get_mut_port(&mut self, position: &[usize; 2], side: Side) -> Result<&mut Option<Port>, PortGridError> {
        match side {
            Side::Up => Ok(&mut self.get_mut(position)?.top.origin_up),
            Side::Down => Ok(&mut self
                .get_mut(&[position[0], position[1].checked_sub(1).ok_or(PortGridError::PositionOutOfBounds)?])?
                .top
                .origin_down),
            Side::Left => Ok(&mut self.get_mut(position)?.left.origin_left),
            Side::Right => Ok(&mut self
                .get_mut(&[position[0] + 1, position[1]])?
                .left
                .origin_right),
        }
    }

    /// Get side based of component grid position\
    /// Ignores ports from its own position
    pub fn get_port(&self, position: &[usize; 2], side: Side) -> Result<&Option<Port>, PortGridError> {
        match side {
            Side::Up => Ok(&self.get(position)?.top.origin_up),
            Side::Down => Ok(&self
                .get(&[position[0], position[1].checked_sub(1).ok_or(PortGridError::PositionOutOfBounds)?])?
                .top
                .origin_down),
            Side::Left => Ok(&self.get(position)?.left.origin_left),
            Side::Right => Ok(&self
                .get(&[position[0] + 1, position[1]])?
                .left
                .origin_right),
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

    pub fn get_mut_port_inside(&mut self, position: &[usize; 2], side: Side) -> Result<&mut Option<Port>, PortGridError> {
        match side {
            Side::Up => Ok(&mut self.get_mut(position)?.top.origin_down),
            Side::Down => Ok(&mut self
                .get_mut(&[position[0], position[1].checked_sub(1).ok_or(PortGridError::PositionOutOfBounds)?])?
                .top
                .origin_up),
            Side::Left => Ok(&mut self.get_mut(position)?.left.origin_right),
            Side::Right => Ok(&mut self
                .get_mut(&[position[0] + 1, position[1]])?
                .left
                .origin_left),
        }
    }
}

#[derive(Debug)]
pub enum PortGridError {
    PositionOutOfBounds,
    IncorrectOrigin,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Port {
    pub checked: bool,
}

impl Port {
    pub fn mark_checked(&mut self, checked: bool) {
        self.checked = checked;
    }
}