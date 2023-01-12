use std::path::PathBuf;

use bevy::prelude::Component;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
pub enum Components {
    WirePiece,
    GateNot,
    GateAnd,
    SignalCopy,
    SignalPassthrough,

}

impl Components {
    pub fn get_path(&self) -> PathBuf {
        let mut p = PathBuf::from("components");
        let s = match self {
            Components::WirePiece => "wire_fake.png",
            Components::GateNot => "gate_not.png",
            Components::GateAnd => "gate_and.png",
            Components::SignalCopy => "signal_copy.png",
            Components::SignalPassthrough => "signal_passthrough.png",
        };
        p.push(s);
        p
    }
}

#[derive(Debug, Component)]
pub struct GridPos(pub u8,pub u8);