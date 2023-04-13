use bevy::{
    prelude::{
        in_state, App, AssetServer, Commands, FromWorld, Handle, Image as BevyImage,
        IntoSystemConfig, Local, NextState, Plugin, Res, ResMut, Resource, World,
    },
    time::Time,
};
use bevy_egui::EguiContexts;
use egui::{plot::Plot, *};

use crate::GameState;
pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_startup_system(configure_ui_state_system)
            .add_system(left_panel.run_if(in_state(GameState::InGame)));
    }
}

pub fn left_panel(
    mut commands: Commands,
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: EguiContexts,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Local<Images>,
    time: Res<Time>,
) {
    // At the moment `CurrentLevel` actually refers to the level to load
    let egui_texture_handle = ui_state
        .egui_texture_handle
        .get_or_insert_with(|| {
            egui_ctx.ctx_mut().load_texture(
                "example",
                egui::ColorImage::example(),
                Default::default(),
            )
        })
        .clone();

    let mut load = false;
    let mut remove = false;
    let mut invert = false;

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
        let exit_button = ui.add(egui::widgets::ImageButton::new(
            *rendered_texture_id,
            [32.0, 32.0],
        ));
        if exit_button.clicked() {
            commands.insert_resource(NextState(Some(GameState::MainMenu)))
        }
    });

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::widgets::Image::new(
                egui_texture_handle.id(),
                egui_texture_handle.size_vec2(),
            ));

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
            });

            ui.add(egui::widgets::Image::new(
                *rendered_texture_id,
                [256.0, 256.0],
            ));

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    TopBottomPanel::bottom("bottom_panel")
        .default_height(20.0)
        .resizable(true)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                ui.label("Lololollololll");
                let button = egui::ImageButton::new(*rendered_texture_id, Vec2::new(100.0, 100.0))
                    .frame(true);
                ui.add(button);
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
                    .view_aspect(3.0)
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

pub fn configure_ui_state_system(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

#[derive(Default, Resource)]
pub struct UiState {
    pub label: String,
    pub value: f32,
    pub inverted: bool,
    pub egui_texture_handle: Option<egui::TextureHandle>,
    pub is_window_open: bool,
}
