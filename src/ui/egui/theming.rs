use std::{fs::File, io::Read};

use bevy::prelude::*;
use bevy_egui::EguiContext;
use egui::{Color32, style::{Widgets, WidgetVisuals}, Stroke, Rounding};

use crate::config::UserSettings;
use super::colours::ColourScheme;

pub struct EguiThemingPlugin;

impl Plugin for EguiThemingPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system(egui_startup);
    }
}

pub fn egui_startup(mut egui_ctx: ResMut<EguiContext>, user_config: Res<UserSettings>, bevy_clear_color: ResMut<ClearColor>) {
    let mut s = String::new();
    File::open(&user_config.theme).unwrap().read_to_string(&mut s).unwrap();
    let colours: ColourScheme = serde_json::from_str(&s).expect("Could not parse user-config.toml");
    dbg!(&colours);
    configure_egui(egui_ctx.ctx_mut(), colours.clone(), bevy_clear_color);
}

pub fn configure_egui(egui_ctx: &egui::Context, colours: ColourScheme, mut bevy_clear_color: ResMut<ClearColor>) {
    bevy_clear_color.0 = Color::rgba_u8(colours.base.0[0], colours.base.0[1], colours.base.0[2], colours.base.0[3]);
    let c = colours;
    egui_ctx.set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        panel_fill: c.mantle.clone().into(),
        code_bg_color: c.mantle.clone().into(),
        window_fill: c.overlay0.clone().into(),
        extreme_bg_color: c.crust.clone().into(),
        faint_bg_color: c.crust.clone().into(),
        hyperlink_color: c.blue.clone().into(),
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_stroke: Stroke::new(5.0, c.crust.clone()),
                fg_stroke: Stroke::new(1.0, c.text.clone()),
                bg_fill: Color32::from_gray(27), // Default
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            inactive: WidgetVisuals {
                bg_fill: c.surface0.clone().into(),      // checkbox background
                bg_stroke: Default::default(),
                fg_stroke: Stroke::new(1.0, c.text.clone()), // button text
                rounding: Rounding::same(2.0),
                expansion: 0.0,
            },
            hovered: WidgetVisuals {
                bg_fill: c.surface1.clone().into(),
                bg_stroke: Stroke::new(1.0, c.surface2.clone()), // e.g. hover over window edge or button
                fg_stroke: Stroke::new(1.5, c.text.clone()),
                rounding: Rounding::same(3.0),
                expansion: 1.0,
            },
            active: WidgetVisuals {
                bg_fill: c.surface2.clone().into(),
                bg_stroke: Stroke::new(1.0, c.surface2.clone()),
                fg_stroke: Stroke::new(2.0, c.text.clone()),
                rounding: Rounding::same(2.0),
                expansion: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    });
    //egui_ctx.ctx_mut().set_style(style)
}