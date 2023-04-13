use bevy::prelude::*;

/// Simulation State
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum SimState {
    #[default]
    Halted,
    Paused,
    Active,
}

#[derive(Debug, Resource, Clone, Copy, Default)]
pub enum RunType {
    #[default]
    None,
    Step(u32),
    Continuous,
}
