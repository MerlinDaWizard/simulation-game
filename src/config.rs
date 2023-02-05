use std::{path::PathBuf, fs::File, io::Read};

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UserSettings>();
    }
}

/// Struct to store user settings. E.g. Theme
#[derive(Resource, Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: PathBuf
}

/// Lets us use .init_resource instead of a start up system.
impl FromWorld for UserSettings {
    fn from_world(_: &mut World) -> Self {
        let mut s = String::new();
        File::open("data/user-config.toml").unwrap().read_to_string(&mut s).unwrap();
        let config: UserSettings = toml::from_str(&s).expect("Could not parse user-config.toml");
        config
    }
}