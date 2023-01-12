use std::path::PathBuf;

use bevy::prelude::{Component, Vec2};
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

    pub fn get_size(&self) -> Vec2 {
        match self {
            Components::WirePiece => Vec2::splat(32.0),
            Components::GateNot => Vec2::splat(32.0),
            Components::GateAnd => Vec2::splat(64.0),
            Components::SignalCopy => Vec2::new(32.0,64.0),
            Components::SignalPassthrough => Vec2::splat(32.0),
        }
    }
}

#[derive(Component)]
pub struct Size(pub Vec2);

#[derive(Debug, Component)]
pub struct GridPos(pub u8,pub u8);