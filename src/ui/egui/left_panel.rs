use bevy::prelude::*;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::ConditionSet;

pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
        .add_startup_system(configure_visuals_system)
        .add_startup_system(configure_ui_state_system);


        app.add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::InGame)
                .with_system(left_panel)
                .into()
        );
    }
}


pub fn left_panel(
    mut ui_state: ResMut<UiState>,
    mut egui_ctx: ResMut<EguiContext>,
    mut rendered_texture_id: Local<egui::TextureId>,
    mut is_initialized: Local<bool>,
    images: Local<Images>, 
) { // At the moment `CurrentLevel` actually refers to the level to load
    let egui_texture_handle = ui_state
    .egui_texture_handle
    .get_or_insert_with(|| {
        egui_ctx
            .ctx_mut()
            .load_texture("example", egui::ColorImage::example(), Default::default())
    })
    .clone();

    let mut load = false;
    let mut remove = false;
    let mut invert = false;

    if !*is_initialized {
        *is_initialized = true;
        *rendered_texture_id = egui_ctx.add_image(images.bevy_icon.clone_weak());
    }

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
}
pub struct Images {
    pub bevy_icon: Handle<Image>,
}

impl FromWorld for Images {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        Self {
            bevy_icon: asset_server.load("bavy.png"),
        }
    }
}

pub fn configure_visuals_system(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        // window_rounding: 0.0.into(),
        ..Default::default()
    });
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