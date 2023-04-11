use bevy::prelude::*;
use crate::{back_to_menu_on_esc, GameState};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            back_to_menu_on_esc,
        ).distributive_run_if(in_state(GameState::Settings)));
    }
}
