use std::path::PathBuf;
use glob::glob;
use bevy::{
    prelude::{
        in_state, App, AssetServer, Commands, FromWorld, Handle, Image as BevyImage,
        IntoSystemConfig, Local, NextState, Plugin, Res, ResMut, Resource, World, EventWriter, State,
    },
    time::Time,
};
use bevy_egui::EguiContexts;
use egui::{plot::Plot, *};

use crate::{GameState, sim::{run::{SimState, RunType}, save_load::{SaveEvent, LoadEvent}, interactions::SelectedComponent, levels::LevelData}, level_select::CurrentLevel};
pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .init_resource::<SaveMenuState>()
            .add_system(main_panels.run_if(in_state(GameState::InGame)))
            .add_system(window_popup.run_if(in_state(GameState::InGame)));
    }
}

fn main_panels(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: EguiContexts,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Local<Images>,
    time: Res<Time>,
    sim_state: Res<State<SimState>>,
    cur_level: Res<CurrentLevel>,
    mut save_menu_state: ResMut<SaveMenuState>,
    mut save_writer: EventWriter<SaveEvent>,
    mut load_writer: EventWriter<LoadEvent>,
    selected_component: ResMut<SelectedComponent>,
    level_data: Option<Res<LevelData>>,
) {
    let sim_halted = sim_state.0 == SimState::Halted;
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
                let save_button = ui.add_enabled(sim_halted, egui::widgets::ImageButton::new(
                    *rendered_texture_id,
                    [32.0, 32.0],
                ));

                let load_button = ui.add_enabled(sim_halted && ui_state.selected_file.is_some(), egui::widgets::ImageButton::new(
                    *rendered_texture_id,
                    [32.0, 32.0],
                ));

                let mut save_dropdown_changed = false;
                let save_dropdown = containers::ComboBox::from_id_source("save_dropdown")
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
                        ui.add_enabled_ui( sim_halted, |ui| {
                            ui.selectable_value(&mut ui_state.selected_file, None, "Create new");
                            for file_path in ui_state.files.clone() {
                                let val_resp = ui.selectable_value(&mut ui_state.selected_file, Some(file_path.clone()), file_path.file_name().unwrap().to_str().unwrap());
                                if val_resp.changed() {
                                    save_dropdown_changed = true;
                                }
                            }
                        });
                    }).response;

                if save_dropdown.clicked_by(PointerButton::Primary) {
                    if let Ok(paths) = glob(&format!("data/levels/user/{}/*.save", cur_level.0.unwrap())) {
                        let p: Vec<PathBuf> = paths.filter_map(|p| p.ok()).collect();
                        ui_state.files = p;
                    }
                }

                if save_dropdown_changed {
                    if let Some(path) = &ui_state.selected_file {
                        load_writer.send(LoadEvent(path.clone()));
                        save_menu_state.open = false;
                    }
                }

                if save_button.clicked_by(PointerButton::Primary) {
                    match &ui_state.selected_file {
                        Some(path) => {save_writer.send(SaveEvent(path.clone()))}
                        None => {
                            save_menu_state.open = true;
                        }
                    }
                }

                if load_button.clicked_by(PointerButton::Primary) {
                    if let Some(path) = &ui_state.selected_file {
                        load_writer.send(LoadEvent(path.clone()));
                    }
                }
            });
        });
    });

    egui::SidePanel::left("left_panel")
        .default_width(250.0)
        .exact_width(250.0)
        .resizable(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            match &selected_component.0 {
                // No component selected. should display problem description
                None => {
                    if let Some(level_data) = level_data {
                        ui.label(RichText::new(format!("Level: {}", cur_level.0.unwrap())).size(25.0).strong().italics());
                        ui.separator();
                        ui.label(RichText::new(level_data.name.clone()).size(20.0).strong());
                        ui.separator();
                        ui.label(level_data.desc.clone());
                    }
                },
                // Should display a brief explanation of the component, a delete button & any options for it
                Some(grid_pos) => {

                }
            }
        });

    TopBottomPanel::bottom("bottom_panel")
        .default_height(200.0)
        .min_height(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
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
    save_button: Handle<BevyImage>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("bavy.png"),
            back_button: asset_server.load("egui/back.png"),
            save_button: asset_server.load("egui/save_button.png"),
        }
    }
}

#[derive(Default, Resource)]
pub struct UiState {
    pub egui_texture_handle: Option<egui::TextureHandle>,
    pub back_button: Option<egui::TextureHandle>,
    pub save_button: Option<egui::TextureHandle>,
    pub files: Vec<PathBuf>,
    pub selected_file: Option<PathBuf>,
}

fn window_popup(
    current_level: Res<CurrentLevel>,
    mut save_menu_state: ResMut<SaveMenuState>,
    mut main_ui_state: ResMut<UiState>,
    mut egui_ctx: EguiContexts,
    sim_state: Res<State<SimState>>,
    mut file_name: Local<String>,
    mut save_writer: EventWriter<SaveEvent>,
) {
    let sim_halted = sim_state.0 == SimState::Halted;
    let mut should_close_window = false;
    // Super high default position to make it into the top right.
    egui::Window::new("Save Window").default_pos(Pos2::new(10000.0,0.0)).fixed_size(Vec2::new(200.0, 50.0)).open(&mut save_menu_state.open).show(egui_ctx.ctx_mut(), |ui| {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label("Save name:");
            ui.text_edit_singleline(&mut *file_name);
        });
        ui.separator(); // Do the line across
        let text = RichText::new("Save").font(FontId { size: 20.0, family: FontFamily::Monospace });
        ui.add_enabled_ui(sim_halted, |ui| {
            let resp = ui.add_sized(ui.available_size(), Button::new(text));
            if resp.clicked() {
                if file_name.len() > 0 {
                    let options = sanitize_filename::Options {
                        replacement: "-",
                        ..Default::default()
                    };

                    let dir = PathBuf::from(format!("data/levels/user/{}", current_level.0.unwrap()));
                    let mut location = dir.join(sanitize_filename::sanitize_with_options(&*file_name, options));
                    location.set_extension("save"); // Check valid path

                    main_ui_state.selected_file = Some(location.clone()); // Select in dropdown
                    save_writer.send(SaveEvent(location)); // Send event
                    should_close_window = true; // Close window
                }
            }
        })
    });

    if should_close_window {save_menu_state.open = false;} // 2 step due to cant mutably borrow save_menu_state while in loop due to double mut borrow
}

#[derive(Resource, Debug, Default)]
struct SaveMenuState {
    open: bool,
}