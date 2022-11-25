use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;

use crate::GameState;

/// Marker for our in-game sprites
#[derive(Component)]
pub struct MySprite;

/// Reset the in-game state when pressing delete
pub fn clear_on_del(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Delete) || kbd.just_pressed(KeyCode::Back) {
        commands.insert_resource(NextState(GameState::InGame));
    }
}

/// Condition system for holding the space bar
pub fn spacebar_pressed(kbd: Res<Input<KeyCode>>) -> bool {
    kbd.pressed(KeyCode::Space)
}

/// Spawn a MySprite entity
pub fn spawn_sprite(mut commands: Commands) {
    let mut rng = thread_rng();
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(rng.gen(), rng.gen(), rng.gen(), 0.5),
            custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(
            rng.gen_range(-420.0..420.0),
            rng.gen_range(-420.0..420.0),
            rng.gen_range(0.0..100.0),
        ),
        ..Default::default()
    }, MySprite));
}

/// Rotate all the sprites
pub fn spin_sprites(mut q: Query<&mut Transform, With<MySprite>>, t: Res<Time>) {
    for mut transform in q.iter_mut() {
        transform.rotate(Quat::from_rotation_z(1.0 * t.delta_seconds()));
    }
}