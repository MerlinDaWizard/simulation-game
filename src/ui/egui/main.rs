use std::{path::PathBuf, cmp};
use egui_extras::{TableBuilder, Column, TableRow};
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

use crate::{GameState, sim::{run::{SimState, RunType}, save_load::{SaveEvent, LoadEvent}, interactions::{SelectedComponent, UpdateComponentEvent}, levels::{LevelData, SimIOPadded, ResultType}, model::{SimulationData, CellState, GridComponent}}, level_select::CurrentLevel};
pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .init_resource::<SaveMenuState>()
            .init_resource::<SimIOPadded>()
            .add_system(main_panels.run_if(in_state(GameState::InGame)))
            .add_system(window_popup.run_if(in_state(GameState::InGame)));
    }
}

fn main_panels(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: EguiContexts,
    images: Local<Images>,
    sim_state: Res<State<SimState>>,
    cur_level: Res<CurrentLevel>,
    mut save_menu_state: ResMut<SaveMenuState>,
    mut save_writer: EventWriter<SaveEvent>,
    mut load_writer: EventWriter<LoadEvent>,
    mut update_component_writer: EventWriter<UpdateComponentEvent>,
    mut selected_component: ResMut<SelectedComponent>,
    mut sim_data: ResMut<SimulationData>,
    level_data: Option<Res<LevelData>>,
    io_data: Option<Res<SimIOPadded>>,
) {
    let sim_halted = sim_state.0 == SimState::Halted;
    // At the moment `CurrentLevel` actually refers to the level to load
    let img_bevy = egui_ctx.add_image(images.bevy_icon.clone_weak());
    let img_back = egui_ctx.add_image(images.back_button.clone_weak());
    let img_save = egui_ctx.add_image(images.save_button.clone_weak());
    let img_load = egui_ctx.add_image(images.load_button.clone_weak());

    let mut panel = egui::SidePanel::right("right_panel")
        .exact_width(250.0)
        // .frame(if sim_halted {Frame::none()} else {Frame::default()})
        .resizable(false);

    if sim_halted {panel = panel.frame(Frame::none());};

    let got_data = io_data.is_some() && level_data.is_some();
    if got_data && sim_halted == false {
        let level_data = level_data.as_ref().unwrap();
        let sim_io = io_data.as_ref().unwrap();
        panel.show(egui_ctx.ctx_mut(), |ui| {
            draw_table(ui,level_data.as_ref(), sim_io.as_ref())
        });
    } else {
        panel.show(egui_ctx.ctx_mut(), |_| {});
    }
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        ui.horizontal_centered(|ui| {
            let exit_button = ui.add(egui::widgets::ImageButton::new(
                img_back,
                [32.0, 32.0],
            ));
            if exit_button.clicked() {
                commands.insert_resource(NextState(Some(GameState::MainMenu2)))
            }

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                let save_button = ui.add_enabled(sim_halted, egui::widgets::ImageButton::new(
                    img_save,
                    [32.0, 32.0],
                ));

                let load_button = ui.add_enabled(sim_halted && ui_state.selected_file.is_some(), egui::widgets::ImageButton::new(
                    img_load,
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
                        selected_component.0 = None;
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
        .default_width(270.0)
        .exact_width(270.0)
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
                    if let CellState::Real(_, component) = &mut sim_data.grid.grid[grid_pos[0]][grid_pos[1]] {
                        let dummy = component.dummy();
                        ui.label(RichText::new(dummy.name()).size(25.0).strong().monospace());
                        ui.separator();
                        ui.label(RichText::new(dummy.desc()).size(12.0).weak());
                        ui.separator();
                        component.gui_options(ui, sim_halted, dummy, grid_pos, &mut update_component_writer);
                    }
                }
            }
        });

    TopBottomPanel::bottom("bottom_panel")
        .default_height(200.0)
        .min_height(200.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                let button = egui::ImageButton::new(img_bevy, Vec2::new(100.0, 100.0)).frame(true);
                let start_test = ui.add(button);
                if start_test.clicked() {
                    commands.insert_resource(NextState(Some(SimState::Building)));
                    commands.insert_resource(RunType::Step(100));
                    // selected_component.0 = None;
                }
                let button = egui::ImageButton::new(img_bevy, Vec2::new(50.0, 50.0)).frame(true);
                let save = ui.add(button);
                if save.clicked() {
                    save_writer.send(SaveEvent(PathBuf::from("data/levels/test.json")))
                }

                let button = egui::ImageButton::new(img_bevy, Vec2::new(50.0, 50.0)).frame(true);
                let load = ui.add_enabled(true, button);
                if load.clicked() {
                    load_writer.send(LoadEvent(PathBuf::from("data/levels/test.json")))
                }
                let sin: plot::PlotPoints = (0..4.0f64.floor() as usize)
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
    load_button: Handle<BevyImage>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("bavy.png"),
            back_button: asset_server.load("exit_button.png"),
            save_button: asset_server.load("egui/save_button.png"),
            load_button: asset_server.load("egui/load_button.png"),
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

fn draw_table(ui: &mut Ui, level_data: &LevelData, io_data: &SimIOPadded) {
    let column_amount = level_data.provided_inputs.len() + level_data.expected_outputs.len() * 2;

    let text_style = egui::TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);

    // Get the values of each type and their lengths
    let provided_in_len = level_data.provided_inputs.values().map(|vec| {vec.len()}).max().unwrap();
    let expected_out_len = io_data.expected_outputs.values().map(|vec| {vec.len()}).max().unwrap();
    let observed_out_len = io_data.observed_outputs.values().map(|vec| {vec.len()}).max().unwrap();
    // Combine all the iterators together into one.
    // Get the maximum length
    let max_length = cmp::max(cmp::max(provided_in_len, expected_out_len), observed_out_len);

    let table = TableBuilder::new(ui)
        .striped(true)
        .cell_layout(Layout::left_to_right(Align::Center))
        .vscroll(true)
        .columns(Column::remainder().clip(true), column_amount);

    table.header(20.0, |mut header| {
        for i in level_data.provided_inputs.keys() {
            header.col(|ui| {
                ui.label(i.as_str());
            });
        }

        for i in io_data.expected_outputs.keys() {
            header.col(|ui| {
                ui.label(format!("Ex: {}", i.as_str()));
            });
        }

        for i in io_data.observed_outputs.keys() {
            header.col(|ui| {
                ui.label(format!("Ob: {}", i));
            });
        }
    })
    .body(|body| {
        body.rows(row_height, max_length, |idx, mut row| {
            // dbg!(&io_data);
            fn to_cell(row: &mut TableRow, val: Option<u8>) {
                let text = match val {
                    Some(num) => num.to_string(),
                    None => String::from("-"),
                };
                row.col(|ui| {ui.label(text);});
            }

            for values in level_data.provided_inputs.values() {
                // println!("provided");
                let val = values.get(idx).copied();
                to_cell(&mut row, val);
            }

            for values in io_data.expected_outputs.values() {
                // println!("expected");
                let val = values.get(idx).and_then(|a| a.clone());
                to_cell(&mut row, val);
            }

            for values in io_data.observed_outputs.values() {
                // println!("Obserrved");
                let val = values.get(idx).and_then(|a| a.clone());
                match val {
                    Some((num, err)) => {
                        match err {
                            ResultType::Incorrect => {row.col(|ui| {ui.label(RichText::new(num.to_string()).color(Color32::RED).strong());});},
                            ResultType::Correct => {row.col(|ui| {ui.label(RichText::new(num.to_string()).color(Color32::GREEN).strong());});},
                        }
                    },
                    None => {
                        row.col(|ui| {ui.label(RichText::new("-"));});
                    },
                };
            }
        })
    });

}