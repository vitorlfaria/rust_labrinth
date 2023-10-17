use crate::{frame::{Drawable, Frame}, NUM_COLS, NUM_ROWS};

use super::{level_tile::{LevelTile, VERTICAL_WALL, BOTTOM_LEFT_CORNER, HORIZONTAL_WALL, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, BOTTOM_RIGHT_CORNER, T_UP, T_RIGHT, T_LEFT, T_DOWN}, level::LevelFactory};

pub struct Level1 {
    pub tiles: Vec<LevelTile>,
}

impl Level1 {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
        }
    }
}

impl LevelFactory for Level1 {
    fn create_level(&mut self) {
        let mut tiles = Vec::new();

        // Base walls
        for i in 1..NUM_ROWS - 1 {
            tiles.push(LevelTile { x: 0, y: i, graphic: VERTICAL_WALL });
            tiles.push(LevelTile { x: NUM_COLS - 1, y: i, graphic: VERTICAL_WALL });
        }
        for i in 1..NUM_COLS - 1 {
            tiles.push(LevelTile { x: i, y: NUM_ROWS - 1, graphic: HORIZONTAL_WALL });
            tiles.push(LevelTile { x: i, y: 0, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 0, y: 0, graphic: TOP_LEFT_CORNER });
        tiles.push(LevelTile { x: NUM_COLS - 1, y: 0, graphic: TOP_RIGHT_CORNER });
        tiles.push(LevelTile { x: 0, y: NUM_ROWS - 1, graphic: BOTTOM_LEFT_CORNER });
        tiles.push(LevelTile { x: NUM_COLS - 1, y: NUM_ROWS - 1, graphic: BOTTOM_RIGHT_CORNER });

        // TOP MIDDLE ===============================================
        tiles.push(LevelTile { x: 5, y: 0, graphic: T_UP });
        tiles.push(LevelTile { x: 5, y: 1, graphic: VERTICAL_WALL });
        tiles.push(LevelTile { x: 5, y: 2, graphic: VERTICAL_WALL });
        tiles.push(LevelTile { x: 5, y: 3, graphic: VERTICAL_WALL });
        tiles.push(LevelTile { x: 5, y: 4, graphic: BOTTOM_LEFT_CORNER });
        for i in 6..24 {
            tiles.push(LevelTile { x: i, y: 4, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 24, y: 4, graphic: TOP_RIGHT_CORNER });
        for i in 5..12 {
            tiles.push(LevelTile { x: 24, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 24, y: 12, graphic: BOTTOM_LEFT_CORNER });
        for i in 25..52 {
            tiles.push(LevelTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 52, y: 12, graphic: BOTTOM_RIGHT_CORNER });
        for i in 1..12 {
            tiles.push(LevelTile { x: 52, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 52, y: 0, graphic: T_UP });

        // TOP RIGHT ===============================================
        tiles.push(LevelTile { x: 58, y: 0, graphic: T_UP });
        for i in 1..12 {
            tiles.push(LevelTile { x: 58, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 58, y: 12, graphic: BOTTOM_LEFT_CORNER });
        for i in 59..NUM_COLS - 1 {
            tiles.push(LevelTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }

        // TOP LEFT ===============================================
        tiles.push(LevelTile { x: NUM_COLS - 1, y: 12, graphic: T_RIGHT });
        tiles.push(LevelTile { x: 0, y: 12, graphic: T_LEFT });
        for i in 1..=16 {
            tiles.push(LevelTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 17, y: 12, graphic: BOTTOM_RIGHT_CORNER });
        for i in 8..=11 {
            tiles.push(LevelTile { x: 17, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 17, y: 7, graphic: TOP_RIGHT_CORNER });
        for i in 1..=16 {
            tiles.push(LevelTile { x: i, y: 7, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 0, y: 7, graphic: T_LEFT });

        // BOTTOM LEFT ===============================================
        tiles.push(LevelTile { x: 0, y: 16, graphic: T_LEFT });
        for i in 1..=32 {
            tiles.push(LevelTile { x: i, y: 16, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: 33, y: 16, graphic: TOP_RIGHT_CORNER });
        for i in 17..NUM_ROWS - 1 {
            tiles.push(LevelTile { x: 33, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 33, y: NUM_ROWS - 1, graphic: T_DOWN });

        // BOTTOM RIGHT ===============================================
        tiles.push(LevelTile { x: 47, y: NUM_ROWS - 1, graphic: T_DOWN });
        for i in 17..NUM_ROWS - 1 {
            tiles.push(LevelTile { x: 47, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(LevelTile { x: 47, y: 16, graphic: TOP_LEFT_CORNER });
        for i in 48..NUM_COLS - 1 {
            tiles.push(LevelTile { x: i, y: 16, graphic: HORIZONTAL_WALL });
        }
        tiles.push(LevelTile { x: NUM_COLS - 1, y: 16, graphic: T_RIGHT });

        self.tiles = tiles;
    }
}

impl Drawable for Level1 {
    fn draw(&self, frame: &mut Frame) {
        for tile in &self.tiles {
            frame[tile.x][tile.y] = tile.graphic.to_string();
        }
    }
}
