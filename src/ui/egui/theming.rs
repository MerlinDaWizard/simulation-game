use std::{fs::File, io::Read};

use bevy::prelude::*;
use bevy_egui::{EguiContexts};
use egui::{Color32, style, epaint};

use crate::config::UserSettings;
use super::colours::ColourScheme;

pub struct EguiThemingPlugin;

impl Plugin for EguiThemingPlugin {
    fn build(&self, app: &mut App) {
       app.add_startup_system(egui_startup);
    }
}

pub fn egui_startup(mut egui_ctx: EguiContexts, user_config: Res<UserSettings>, bevy_clear_color: ResMut<ClearColor>) {
    let mut s = String::new();
    File::open(&user_config.theme).unwrap().read_to_string(&mut s).unwrap();
    let colours: ColourScheme = serde_json::from_str(&s).expect("Could not parse user-config.toml");
    //dbg!(&colours);
    //catppuccin_egui::set_theme(&egui_ctx.ctx_mut(), catppuccin_egui::MACCHIATO);
    configure_egui(egui_ctx.ctx_mut(), colours.clone(), bevy_clear_color);
}

/// Colour bindings revamped from https://github.com/catppuccin/egui
pub fn configure_egui(egui_ctx: &egui::Context, colours: ColourScheme, mut bevy_clear_color: ResMut<ClearColor>) {
    bevy_clear_color.0 = Color::rgba_u8(colours.base.0[0], colours.base.0[1], colours.base.0[2], colours.base.0[3]);
    let c = colours;
    let old = egui_ctx.style().visuals.clone();
    egui_ctx.set_visuals(egui::Visuals {
        override_text_color: Some(c.text.into()),
        hyperlink_color: c.rosewater.into(),
        faint_bg_color: c.surface0.into(),
        extreme_bg_color: c.crust.into(),
        code_bg_color: c.mantle.into(),
        warn_fg_color: c.peach.into(),
        error_fg_color: c.maroon.into(),
        window_fill: c.base.into(),
        panel_fill: c.base.into(),
        window_stroke: egui::Stroke {
            color: c.overlay1.into(),
            ..old.window_stroke
        },
        widgets: style::Widgets {
            noninteractive: make_widget_visual(old.widgets.noninteractive, &c, c.base.into()),
            inactive: make_widget_visual(old.widgets.inactive, &c, c.surface0.into()),
            hovered: make_widget_visual(old.widgets.hovered, &c, c.surface2.into()),
            active: make_widget_visual(old.widgets.active, &c, c.surface1.into()),
            open: make_widget_visual(old.widgets.open, &c, c.surface0.into()),
        },
        selection: style::Selection {
            bg_fill:
                (Color32::from(c.blue))
                .linear_multiply(if c.name == "Latte" { 0.4 } else { 0.2 }),
            stroke: egui::Stroke {
                color: c.overlay1.into(),
                ..old.selection.stroke
            },
        },
        window_shadow: epaint::Shadow {
            color: c.base.into(),
            ..old.window_shadow
        },
        popup_shadow: epaint::Shadow {
            color: c.base.into(),
            ..old.popup_shadow
        },
        ..old
    });
    //egui_ctx.ctx_mut().set_style(style)
}

/// Refer to the catppuccin-license file.
fn make_widget_visual(
    old: style::WidgetVisuals,
    theme: &ColourScheme,
    bg_fill: egui::Color32,
) -> style::WidgetVisuals {
    style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: theme.overlay1.into(),
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: theme.text.into(),
            ..old.fg_stroke
        },
        ..old
    }
}