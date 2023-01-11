//! MODIFIED VERSION TO DEMONSTRATE ONLY WORKING WITH SINGLE SPRITE
//! Demonstrates how to use the bevy_sprite picking backend.
//!
//! You must enable the `backend_sprite` or `all` features.

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_startup_system(setup)
        .add_system(move_sprite)
        .run();
}

fn move_sprite(time: Res<Time>, mut sprite: Query<&mut Transform, With<Sprite>>) {
    for mut transform in sprite.iter_mut() {
        transform.translation.y = 200.0 * (time.elapsed_seconds() * 2.0).sin()
    }

}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bavy.png"),
        sprite: Sprite { // Style image blue just to make things clear
            color: Color::BLUE,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-200.0,0.0,1.0),
            ..Default::default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bavy.png"),
        transform: Transform {
            translation: Vec3::new(200.0,0.0,1.0),
            ..Default::default()
        },
        ..default()
    });
}
