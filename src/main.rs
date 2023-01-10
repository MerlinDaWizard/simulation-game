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

#[derive(Component)]
struct Mover(i32);

fn move_sprite(time: Res<Time>, mut sprite: Query<(&mut Transform, &Mover), With<Sprite>>) {
    for (mut transform, mover) in sprite.iter_mut() {
        let new = Vec2 {
            x: 200.0 * (time.elapsed_seconds().sin() * mover.0 as f32),
            y: 200.0 * (time.elapsed_seconds() * 2.0).sin(),
        };
        transform.translation.x = new.x;
        transform.translation.y = new.y;
    }

}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("grid.png"),
        ..default()
    }, Mover(-1)));

    commands.spawn((SpriteBundle {
        texture: asset_server.load("grid.png"),
        ..default()
    }, Mover(1)));
}
