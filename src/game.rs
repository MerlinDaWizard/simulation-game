use crate::components::placement::Size;
use crate::level_select::CurrentLevel;
use crate::{ui, MainTextureAtlas};
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub const GRID_CELL_SIZE: usize = 32;
pub const GRID_CELL_AMOUNT_WIDTH: u8 = 7;
pub const GRID_CELL_AMOUNT_HEIGHT: u8 = 7;

/// Resource to facilitate changing grid sizes
#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct GridSize(pub [usize; 2]);

impl Default for GridSize {
    fn default() -> Self {
        Self([0, 0])
    }
}

/// Root component for this screen
#[derive(Component)]
pub struct GameRoot;

#[derive(Component)]
pub struct PlacementGridEntity;

/// Sets up screen using flex boxies and loads components etc.
pub fn setup_screen(
    mut commands: Commands,
    ass: Res<AssetServer>,
    _level: Res<CurrentLevel>,

    atlases: Res<Assets<TextureAtlas>>,
    main_atlas: Res<MainTextureAtlas>,
) {
    // At the moment `CurrentLevel` actually refers to the level to load
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3 {
                    x: -60.0,
                    y: 35.0,
                    z: 10.0,
                },
                //scale: Vec3::splat(2.0),
                ..Default::default()
            },
            texture: ass.load("grid.png"),
            ..Default::default()
        },
        GameRoot,
        Name::new("Placement Grid"),
        PlacementGridEntity,
        Size(UVec2::new(
            (GRID_CELL_SIZE * GRID_CELL_AMOUNT_WIDTH as usize) as u32,
            (GRID_CELL_SIZE * GRID_CELL_AMOUNT_HEIGHT as usize) as u32,
        )),
    ));

    ui::textbox::ProgramBox::new(&mut commands, &ass, &atlases, &main_atlas, "A1", GameRoot);
    ui::textbox::ProgramBox::new(&mut commands, &ass, &atlases, &main_atlas, "A2", GameRoot);
}

/// Unit component to mark an entity as interactable for the click_system
#[derive(Component)]
pub struct Interactable;
