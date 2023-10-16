#![allow(dead_code)]

pub struct LevelTile {
    pub x: usize,
    pub y: usize,
    pub graphic: char,
}

pub const HORIZONTAL_WALL: char = '═';
pub const VERTICAL_WALL: char = '║';
pub const TOP_LEFT_CORNER: char = '╔';
pub const TOP_RIGHT_CORNER: char = '╗';
pub const BOTTOM_LEFT_CORNER: char = '╚';
pub const BOTTOM_RIGHT_CORNER: char = '╝';
pub const CROSS: char = '╬';
pub const T_UP: char = '╦';
pub const T_DOWN: char = '╩';
pub const T_LEFT: char = '╠';
pub const T_RIGHT: char = '╣';
