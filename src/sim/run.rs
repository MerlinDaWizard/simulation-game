use bevy::prelude::*;

/// Simulation State
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum SimState {
    #[default]
    Halted,
    Paused,
    Active,
}

#[derive(Debug, Resource, Clone, Default)]
pub enum RunType {
    #[default]
    None,
    Step(u32),
    Continuous,
}

impl RunType {
    pub fn tick(&mut self) -> bool {
        match self {
            RunType::None => todo!(),
            RunType::Step(_) => todo!(),
            RunType::Continuous => todo!(),

        }
    }
}