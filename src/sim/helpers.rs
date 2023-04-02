use enum_map::Enum;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Enum)]
pub enum Side {
    Up,
    Down,
    Left,
    Right,
}

impl Side {
    pub fn as_offset(&self) -> [isize; 2] {
        match self {
            Side::Up => [0,1],
            Side::Down => [0,-1],
            Side::Left => [-1,0],
            Side::Right => [1,0],
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Side::Up => "up",
            Side::Down => "down",
            Side::Left => "left",
            Side::Right => "right",
        }
    }
}

pub fn combine_offset(pos: &[usize; 2], offset: &[isize; 2]) -> Option<[usize; 2]> {
    let new_pos = [pos[0].checked_add_signed(offset[0]), pos[1].checked_add_signed(offset[1])];
    // I feel like there is a better way to do this doing more funcitonal style programming but oh well.
    let mut option_pos = [0usize; 2];
    for (i,x) in new_pos.iter().enumerate() {
        if let Some(v) = x {
            option_pos[i] = *v;
        } else {
            return None;
        }
    }
    return Some(option_pos);
}