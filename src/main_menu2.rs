use bevy::{prelude::*, window::close_on_esc};
use bevy_egui::EguiContext;
use egui::{Frame, Color32, style::Margin};
use iyes_loopless::prelude::ConditionSet;

use crate::ui::egui::main::Images;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::MainMenu2)
                .with_system(close_on_esc)
                .with_system(main_menu)
                .into()
        );
    }
}

fn main_menu(
    mut commands: Commands,
    mut egui_ctx: ResMut<EguiContext>,
    images: Local<MainMenuImages>,
) {
    let background = egui_ctx.add_image(images.background.clone_weak());
    egui::CentralPanel::default().frame(Frame::default().outer_margin(Margin::same(0.0))).show(egui_ctx.ctx_mut(), |ui| {
        ui.image(background, ui.available_size());
        //ui.label("Test!");
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
