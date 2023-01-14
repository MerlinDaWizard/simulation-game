use bevy::prelude::*;

use super::shared::GridComponent;

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GateNot {
    pub grid_pos: UVec2,
}

impl GridComponent for GateNot {
    fn get_grid_pos(&self) -> UVec2 {
        self.grid_pos
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct GateAnd {
    pub grid_pos: UVec2,
}

impl GridComponent for GateAnd {
    fn get_grid_pos(&self) -> UVec2 {
        self.grid_pos
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SignalCopy {
    pub grid_pos: UVec2,
}

impl GridComponent for SignalCopy {
    fn get_grid_pos(&self) -> UVec2 {
        self.grid_pos
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct SignalPassthrough {
    pub grid_pos: UVec2,
}

impl GridComponent for SignalPassthrough {
    fn get_grid_pos(&self) -> UVec2 {
        self.grid_pos
    }
}