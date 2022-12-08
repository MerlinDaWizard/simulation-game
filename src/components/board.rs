use bevy::prelude::*;

#[derive(Bundle)]
struct GridBundle {
    #[bundle]
    sprite: SpriteBundle,
    grid: GridComponents
}

/// Matrix of components
#[derive(Component)]
struct GridComponents(Vec<Vec<Components>>);