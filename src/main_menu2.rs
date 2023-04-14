use crate::GameState;
use bevy::{
    app::AppExit,
    prelude::{
        in_state, App, AssetServer, Commands, EventWriter, FromWorld, Handle, Image,
        IntoSystemConfigs, Local, NextState, Plugin, World,
    },
    window::close_on_esc,
};
use bevy_egui::EguiContexts;
use egui::{
    pos2, style::Margin, vec2, Align2, Color32, Frame, Rect as EguiRect, RichText, Stroke, Vec2,
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (close_on_esc, main_menu).distributive_run_if(in_state(GameState::MainMenu2)),
        );
    }
}

fn main_menu(
    mut commands: Commands,
    mut egui_ctx: EguiContexts,
    images: Local<MainMenuImages>,
    mut exit: EventWriter<AppExit>,
) {
    let background = egui_ctx.add_image(images.background.clone_weak());
    let ctx_mut = egui_ctx.ctx_mut();

    egui::CentralPanel::default()
        .frame(Frame::default().outer_margin(Margin::same(0.0)))
        .show(ctx_mut, |ui| {
            let uv = EguiRect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
            ui.painter().image(
                background,
                ui.available_rect_before_wrap(),
                uv,
                Color32::WHITE,
            );
            egui::Area::new("buttons")
                .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
                .show(ctx_mut, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.scope(|ui| {
                            ui.style_mut().visuals.override_text_color = None;
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::DARK_BLUE;
                            ui.style_mut().visuals.widgets.inactive.fg_stroke = Stroke { width: 3.0, color: Color32::LIGHT_GREEN };

                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =Color32::from_rgb(0, 0, 100);
                            ui.style_mut().visuals.widgets.hovered.fg_stroke = Stroke { width: 3.0, color: Color32::LIGHT_GREEN };

                            ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_rgb(0, 0, 62);

                            let text = RichText::new("Play").font(egui::FontId { size: 46., family: egui::FontFamily::Monospace });
                            let button_play = ui.add_sized(Vec2::new(350., 75.), egui::Button::new(text));
                            if button_play.clicked() {
                                commands.insert_resource(NextState(Some(GameState::InGame)));
                            }

                            ui.allocate_space(vec2(10., 10.));
                            let text = RichText::new("Level Select").font(egui::FontId { size: 46., family: egui::FontFamily::Monospace });
                            let button_lvl = ui.add_sized(Vec2::new(350., 75.), egui::Button::new(text));
                            if button_lvl.clicked() {
                                commands.insert_resource(NextState(Some(GameState::LevelsMenu)));
                            }

                            ui.allocate_space(vec2(10., 10.));
                            let text = RichText::new("Settings").font(egui::FontId { size: 46., family: egui::FontFamily::Monospace });
                            let button_settings = ui.add_sized(Vec2::new(350., 75.), egui::Button::new(text));
                            if button_settings.clicked() {
                                commands.insert_resource(NextState(Some(GameState::Settings)));
                            }

                            ui.allocate_space(vec2(10., 10.));
                            let text = RichText::new("Quit").font(egui::FontId { size: 46., family: egui::FontFamily::Monospace });
                            let button_quit = ui.add_sized(Vec2::new(350., 75.), egui::Button::new(text));
                            if button_quit.clicked() {
                                exit.send(AppExit);
                            }
                        })
                    });
                });
        });
}

struct MainMenuImages {
    pub background: Handle<Image>,
}

impl FromWorld for MainMenuImages {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            background: asset_server.load("background.jpg"),
        }
    }
}
