use std::path::PathBuf;
use glob::glob;
use bevy::{
    prelude::{
        in_state, App, AssetServer, Commands, FromWorld, Handle, Image as BevyImage,
        IntoSystemConfig, Local, NextState, Plugin, Res, ResMut, Resource, World, debug, EventWriter,
    },
    time::Time,
};
use bevy_egui::EguiContexts;
use egui::{plot::Plot, *};

use crate::{GameState, sim::{run::{SimState, RunType}, save_load::{SaveEvent, LoadEvent}}, level_select::CurrentLevel};
pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_system(main_panels.run_if(in_state(GameState::InGame)));
    }
}

pub fn main_panels(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: EguiContexts,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Local<Images>,
    time: Res<Time>,
    cur_level: Res<CurrentLevel>,
    mut save_writer: EventWriter<SaveEvent>,
    mut load_writer: EventWriter<LoadEvent>,
) {
    // At the moment `CurrentLevel` actually refers to the level to load
    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = egui_ctx.add_image(images.bevy_icon.clone_weak());
    }
    egui::SidePanel::right("right_panel")
        .exact_width(250.0)
        .frame(Frame::none())
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {});

    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal_centered(|ui| {
            let exit_button = ui.add(egui::widgets::ImageButton::new(
                *rendered_texture_id,
                [32.0, 32.0],
            ));
            if exit_button.clicked() {
                commands.insert_resource(NextState(Some(GameState::MainMenu2)))
            }


            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                let resp = containers::ComboBox::from_id_source("save_dropdown")
                    .width(300.0)
                    .selected_text(
                        RichText::new(match &ui_state.selected_file {
                            None => "create new",
                            Some(p) => p.file_name().unwrap().to_str().unwrap()
                        }).size(28.0)
                        .family(FontFamily::Monospace)
                    )
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(&mut ui_state.selected_file, None, "Create new");
                        for file_path in ui_state.files.clone() {
                            ui.selectable_value(&mut ui_state.selected_file, Some(file_path.clone()), file_path.file_name().unwrap().to_str().unwrap());
                        }
                    }).response;

                if resp.clicked_by(PointerButton::Primary) {
                    if let Ok(paths) = glob(&format!("data/levels/user/{}/*.save", cur_level.0.unwrap())) {
                        let p: Vec<PathBuf> = paths.filter_map(|p| p.ok()).collect();
                        ui_state.files = p;
                    }
                }
            });
        });
    });

    egui::SidePanel::left("left_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
        });

    TopBottomPanel::bottom("bottom_panel")
        .default_height(200.0)
        .min_height(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label("Lololollololll");
                let button = egui::ImageButton::new(*rendered_texture_id, Vec2::new(100.0, 100.0)).frame(true);
                let start_test = ui.add(button);
                if start_test.clicked() {
                    commands.insert_resource(NextState(Some(SimState::Building)));
                    commands.insert_resource(RunType::Step(100));
                }
                let button = egui::ImageButton::new(*rendered_texture_id, Vec2::new(50.0, 50.0)).frame(true);
                let save = ui.add(button);
                if save.clicked() {
                    save_writer.send(SaveEvent(PathBuf::from("data/levels/test.json")))
                }

                let button = egui::ImageButton::new(*rendered_texture_id, Vec2::new(50.0, 50.0)).frame(true);
                let load = ui.add_enabled(true, button);
                if load.clicked() {
                    load_writer.send(LoadEvent(PathBuf::from("data/levels/test.json")))
                }
                let sin: plot::PlotPoints = (0..time.elapsed_seconds_f64().floor() as usize)
                    .flat_map(|i| {
                        //let x = i as f64 * 0.01;
                        let n = i as f64 * 0.05;
                        [
                            [i as f64, (n.sin() * 100.0).round()],
                            [(i + 1) as f64, (n.sin() * 100.0).round()],
                        ]
                    })
                    .collect();
                let line = plot::Line::new(sin);

                let x_fmt =
                    |x: f64, _range: &std::ops::RangeInclusive<f64>| format!("Tick: {}", x.floor());

                let label_fmt = |_s: &str, val: &plot::PlotPoint| {
                    format!(
                        "Tick {}\n{}",
                        val.x.floor() as usize,
                        val.y.round() as usize
                    )
                };

                Plot::new("graph")
                    .x_axis_formatter(x_fmt)
                    .label_formatter(label_fmt)
                    // .view_aspect(3.0)
                    //.center_y_axis(true)
                    .include_y(100.0)
                    .include_y(0.0)
                    .auto_bounds_y()
                    .legend(plot::Legend::default())
                    .show(ui, |plot_ui| plot_ui.line(line.name("Input")));
            })
        });
}
pub struct Images {
    bevy_icon: Handle<BevyImage>,
    back_button: Handle<BevyImage>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("bavy.png"),
            back_button: asset_server.load("egui/back.png"),
        }
    }
}

#[derive(Default, Resource)]
pub struct UiState {
    pub egui_texture_handle: Option<egui::TextureHandle>,
    pub back_button: Option<egui::TextureHandle>,
    pub files: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
}
