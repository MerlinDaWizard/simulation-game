#[derive(Debug, Component)]
struct GridPos(NonZeroU8,NonZeroU8);

#[derive(Bundle)]
struct WireBundle {
    sprite: SpriteBundle,
    #[Bundle]
    grid_pos: GridPos,
}

#[derive(Bundle)]
struct GridBundle {
    grid: Vec<Vec<Components>>
}

#[derive(Component)]
enum Components {
    Wires(WireBundle),
    // Other components eventually here aswell
}

