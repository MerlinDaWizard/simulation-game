use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LevelData {
    pub grid_width: usize,
    pub grid_height: usize,
}
