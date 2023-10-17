use crate::{frame::{Drawable, Frame}, NUM_COLS, NUM_ROWS};

use super::{wall_tile::{WallTile, VERTICAL_WALL, BOTTOM_LEFT_CORNER, HORIZONTAL_WALL, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, BOTTOM_RIGHT_CORNER}, level::LevelFactory};

pub struct Level {
    pub tiles: Vec<WallTile>,
}

impl Level {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
        }
    }
}

impl LevelFactory for Level {
    fn create_level(&mut self) {
        let mut tiles = Vec::new();

        // Walls
        for i in 1..NUM_ROWS - 1 {
            tiles.push(WallTile { x: 1, y: i, graphic: VERTICAL_WALL });
            tiles.push(WallTile { x: NUM_COLS - 2, y: i, graphic: VERTICAL_WALL });
        }
        for i in 1..NUM_COLS - 1 {
            tiles.push(WallTile { x: i, y: NUM_ROWS - 2, graphic: HORIZONTAL_WALL });
            tiles.push(WallTile { x: i, y: 1, graphic: HORIZONTAL_WALL });
        }

        // Corners
        tiles.push(WallTile { x: 1, y: 1, graphic: TOP_LEFT_CORNER });
        tiles.push(WallTile { x: NUM_COLS - 2, y: 1, graphic: TOP_RIGHT_CORNER });
        tiles.push(WallTile { x: 1, y: NUM_ROWS - 2, graphic: BOTTOM_LEFT_CORNER });
        tiles.push(WallTile { x: NUM_COLS - 2, y: NUM_ROWS - 2, graphic: BOTTOM_RIGHT_CORNER });

        self.tiles = tiles;
    }
}

impl Drawable for Level {
    fn draw(&self, frame: &mut Frame) {
        for tile in &self.tiles {
            frame[tile.x][tile.y] = tile.graphic.to_string();
        }
    }
}
