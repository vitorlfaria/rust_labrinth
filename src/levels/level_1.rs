use crate::{frame::{Drawable, Frame}, NUM_COLS, NUM_ROWS};

use super::level_tile::{LevelTile, T_DOWN, VERTICAL_WALL, BOTTOM_LEFT_CORNER, HORIZONTAL_WALL, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, BOTTOM_RIGHT_CORNER};

pub struct Level {
    pub tiles: Vec<LevelTile>,
}

impl Level {
    pub fn new() -> Self {
        let mut tiles = Vec::new();

        // Walls
        for i in 1..NUM_ROWS - 1 {
            tiles.push(LevelTile { x: 0, y: i, graphic: VERTICAL_WALL });
            tiles.push(LevelTile { x: NUM_COLS - 1, y: i, graphic: VERTICAL_WALL });
        }
        for i in 1..NUM_COLS - 1 {
            tiles.push(LevelTile { x: i, y: NUM_ROWS - 1, graphic: HORIZONTAL_WALL });
            tiles.push(LevelTile { x: i, y: 0, graphic: HORIZONTAL_WALL });
        }

        // Corners
        tiles.push(LevelTile { x: 0, y: 0, graphic: TOP_LEFT_CORNER });
        tiles.push(LevelTile { x: NUM_COLS - 1, y: 0, graphic: TOP_RIGHT_CORNER });
        tiles.push(LevelTile { x: 0, y: NUM_ROWS - 1, graphic: BOTTOM_LEFT_CORNER });
        tiles.push(LevelTile { x: NUM_COLS - 1, y: NUM_ROWS - 1, graphic: BOTTOM_RIGHT_CORNER });

        Self {
            tiles,
        }
    }
}

impl Drawable for Level {
    fn draw(&self, frame: &mut Frame) {
        for tile in &self.tiles {
            frame[tile.x][tile.y] = tile.graphic.to_string();
        }
    }
}
