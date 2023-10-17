use crossterm::style::Stylize;

use crate::{
    frame::{Drawable, Frame},
    NUM_COLS, NUM_ROWS,
};

use super::{
    level_factory::LevelFactory,
    wall_tile::{
        WallTile, BOTTOM_LEFT_CORNER, BOTTOM_RIGHT_CORNER, HORIZONTAL_WALL, TOP_LEFT_CORNER,
        TOP_RIGHT_CORNER, VERTICAL_WALL,
    },
    door_tile::DoorTile,
};

pub struct Level2 {
    pub tiles: Vec<WallTile>,
    pub doors: Vec<DoorTile>,
}

impl Level2 {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            doors: Vec::new(),
        }
    }
}

impl LevelFactory for Level2 {
    fn create_level(&mut self) {
        let mut tiles = Vec::new();
        let mut doors = Vec::new();

        // Base walls
        for i in 1..NUM_ROWS - 1 {
            tiles.push(WallTile { x: 0, y: i, graphic: VERTICAL_WALL });
            tiles.push(WallTile { x: NUM_COLS - 1, y: i, graphic: VERTICAL_WALL });
        }
        for i in 1..NUM_COLS - 1 {
            tiles.push(WallTile { x: i, y: NUM_ROWS - 1, graphic: HORIZONTAL_WALL });
            tiles.push(WallTile { x: i, y: 0, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 0, y: 0, graphic: TOP_LEFT_CORNER });
        tiles.push(WallTile { x: NUM_COLS - 1, y: 0, graphic: TOP_RIGHT_CORNER });
        tiles.push(WallTile { x: 0, y: NUM_ROWS - 1, graphic: BOTTOM_LEFT_CORNER });
        tiles.push(WallTile { x: NUM_COLS - 1, y: NUM_ROWS - 1, graphic: BOTTOM_RIGHT_CORNER });

        // DOORS ======================================================
        doors.push(DoorTile { x: NUM_COLS - 1, y: 13, to_level: 1, is_to_side: true });
        doors.push(DoorTile { x: NUM_COLS - 1, y: 14, to_level: 1, is_to_side: true });
        doors.push(DoorTile { x: NUM_COLS - 1, y: 15, to_level: 1, is_to_side: true });

        self.tiles = tiles;
        self.doors = doors;
    }
}

impl Drawable for Level2 {
    fn draw(&self, frame: &mut Frame) {
        for tile in &self.tiles {
            frame[tile.x][tile.y] = tile.graphic.to_string();
        }
        for door in &self.doors {
            frame[door.x][door.y] = "▓".yellow().to_string();
        }
    }
}
