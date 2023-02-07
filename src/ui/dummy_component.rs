use bevy::sprite::Anchor;
use bevy::{prelude::*, core::Name};
use iyes_loopless::prelude::*;
use strum::IntoEnumIterator;
use crate::MainTextureAtlas;
use crate::{components::shared::*};
use crate::ui::shared::*;
use crate::components::shared::Size;
pub struct ComponentTrayPlugin;

impl Plugin for ComponentTrayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(crate::GameState::InGame, enter_system)
            .add_system_set(
            ConditionSet::new()
                .run_in_state(crate::GameState::InGame)
                // .with_system(ui_example_system)
                .into()
        );
    }
}

fn enter_system(mut commands: Commands, atlases: Res<Assets<TextureAtlas>>, main_atlas: Res<MainTextureAtlas>) {
    let atlas = atlases.get(&main_atlas.handle).unwrap();
    let mut current_down = -150.0;
    for comp in Components::iter() {
        let size = comp.get_size();
        //let texture: Handle<Image> = ass.load(comp.get_path());
        let sprite_idx = comp.get_sprite_index(atlas);
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.anchor = Anchor::BottomLeft;
        commands.spawn((
            SpriteSheetBundle  {
                sprite: sprite,
                transform: Transform {
                    translation: Vec3 { x: 225.0, y: current_down, z: 20.0},
                    //scale: Vec3::splat(SCALE),
                    ..Default::default()
                },
                texture_atlas: main_atlas.handle.clone(),
                ..Default::default()
            },
            Name::new(format!("Dummy Component - {}", comp.get_sprite_name())),
            GridLock::new(),
            crate::game::GameRoot,
            Draggable::new(),
            DragTypeReturn::new(),
            DragOpacity(0.75),
            Size(comp.get_size()),
            ComponentLink(comp),
        ));
        current_down += size.y + 10.0;
    }
}

#[derive(Component)]
pub struct TrayComponent;

#[derive(Component)]
pub struct ComponentLink(pub Components);

#[derive(Component)]
pub struct GridLock{
    pub grab_part: Vec2
}

impl GridLock {
    pub fn new() -> Self {
        GridLock { grab_part: Vec2::ZERO }
    }

}