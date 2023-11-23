use crossterm::style::Stylize;

use crate::{utils::frame::{Drawable, Frame}, NUM_COLS, NUM_ROWS, items::key::Key};

use super::{wall_tile::{WallTile, VERTICAL_WALL, BOTTOM_LEFT_CORNER, HORIZONTAL_WALL, TOP_LEFT_CORNER, TOP_RIGHT_CORNER, BOTTOM_RIGHT_CORNER, T_UP, T_RIGHT, T_LEFT, T_DOWN}, level_factory::LevelFactory, door_tile::DoorTile};

pub struct Level1 {
    pub tiles: Vec<WallTile>,
    pub doors: Vec<DoorTile>,
    pub keys: Vec<Key>,
    pub enemy: (usize, usize),
    pub patrol_points: Vec<(usize, usize)>,
}

impl Level1 {
    pub fn new() -> Self {
        let routes = Self::generate_patrol_points();
        Self {
            tiles: Vec::new(),
            doors: Vec::new(),
            keys: Vec::new(),
            enemy: (10, 15),
            patrol_points: routes,
        }
    }

    fn generate_patrol_points() -> Vec<(usize, usize)> {
        let mut patrol_points = Vec::new();
        
        patrol_points.push((3, 2));
        patrol_points.push((2, 15));
        patrol_points.push((55, 2));
        patrol_points.push((70, 14));
        patrol_points.push((40, 22));

        patrol_points
    }
}

impl LevelFactory for Level1 {
    fn create_level(&mut self) {
        let mut tiles = Vec::new();
        let mut doors = Vec::new();
        let mut keys = Vec::new();

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

        // TOP MIDDLE ===============================================
        tiles.push(WallTile { x: 5, y: 0, graphic: T_UP });
        tiles.push(WallTile { x: 5, y: 1, graphic: VERTICAL_WALL });
        tiles.push(WallTile { x: 5, y: 2, graphic: VERTICAL_WALL });
        tiles.push(WallTile { x: 5, y: 3, graphic: VERTICAL_WALL });
        tiles.push(WallTile { x: 5, y: 4, graphic: BOTTOM_LEFT_CORNER });
        for i in 6..24 {
            tiles.push(WallTile { x: i, y: 4, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 24, y: 4, graphic: TOP_RIGHT_CORNER });
        for i in 5..12 {
            tiles.push(WallTile { x: 24, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 24, y: 12, graphic: BOTTOM_LEFT_CORNER });
        for i in 25..52 {
            tiles.push(WallTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 52, y: 12, graphic: BOTTOM_RIGHT_CORNER });
        for i in 1..12 {
            tiles.push(WallTile { x: 52, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 52, y: 0, graphic: T_UP });

        // TOP RIGHT ===============================================
        tiles.push(WallTile { x: 58, y: 0, graphic: T_UP });
        for i in 1..12 {
            tiles.push(WallTile { x: 58, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 58, y: 12, graphic: BOTTOM_LEFT_CORNER });
        for i in 59..NUM_COLS - 1 {
            tiles.push(WallTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }

        // TOP LEFT ===============================================
        tiles.push(WallTile { x: NUM_COLS - 1, y: 12, graphic: T_RIGHT });
        tiles.push(WallTile { x: 0, y: 12, graphic: T_LEFT });
        for i in 1..=16 {
            tiles.push(WallTile { x: i, y: 12, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 17, y: 12, graphic: BOTTOM_RIGHT_CORNER });
        for i in 8..=11 {
            tiles.push(WallTile { x: 17, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 17, y: 7, graphic: TOP_RIGHT_CORNER });
        for i in 1..=16 {
            tiles.push(WallTile { x: i, y: 7, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 0, y: 7, graphic: T_LEFT });

        // BOTTOM LEFT ===============================================
        tiles.push(WallTile { x: 0, y: 16, graphic: T_LEFT });
        for i in 1..=32 {
            tiles.push(WallTile { x: i, y: 16, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: 33, y: 16, graphic: TOP_RIGHT_CORNER });
        for i in 17..NUM_ROWS - 1 {
            tiles.push(WallTile { x: 33, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 33, y: NUM_ROWS - 1, graphic: T_DOWN });

        // BOTTOM RIGHT ===============================================
        tiles.push(WallTile { x: 47, y: NUM_ROWS - 1, graphic: T_DOWN });
        for i in 17..NUM_ROWS - 1 {
            tiles.push(WallTile { x: 47, y: i, graphic: VERTICAL_WALL });
        }
        tiles.push(WallTile { x: 47, y: 16, graphic: TOP_LEFT_CORNER });
        for i in 48..NUM_COLS - 1 {
            tiles.push(WallTile { x: i, y: 16, graphic: HORIZONTAL_WALL });
        }
        tiles.push(WallTile { x: NUM_COLS - 1, y: 16, graphic: T_RIGHT });

        // DOORS ======================================================
        // To level 2
        doors.push(DoorTile { x: 0, y: 13, to_level: 2, is_to_side: true, required_key: None });
        doors.push(DoorTile { x: 0, y: 14, to_level: 2, is_to_side: true, required_key: None });
        doors.push(DoorTile { x: 0, y: 15, to_level: 2, is_to_side: true, required_key: None });

        // To level 3
        doors.push(DoorTile { x: 1, y: 0, to_level: 3, is_to_side: false, required_key: Some("3".to_string()) });
        doors.push(DoorTile { x: 2, y: 0, to_level: 3, is_to_side: false, required_key: Some("3".to_string()) });
        doors.push(DoorTile { x: 3, y: 0, to_level: 3, is_to_side: false, required_key: Some("3".to_string()) });
        doors.push(DoorTile { x: 4, y: 0, to_level: 3, is_to_side: false, required_key: Some("3".to_string()) });

        // KEYS ========================================================
        keys.push(Key { x: 3, y: 5, name: "3".to_string() });

        self.tiles = tiles;
        self.doors = doors;
        self.keys = keys;
    }
}

impl Drawable for Level1 {
    fn draw(&self, frame: &mut Frame, _render_area: &Vec<Vec<(usize, usize)>>) {
        for tile in &self.tiles {
            if _render_area[1].contains(&(tile.x, tile.y)) {
                frame[tile.x][tile.y] = tile.graphic.dark_grey().to_string();
            }
            if _render_area[0].contains(&(tile.x, tile.y)) {
                frame[tile.x][tile.y] = tile.graphic.to_string();
            }
        }
        for door in &self.doors {
            if _render_area[1].contains(&(door.x, door.y)) {
                frame[door.x][door.y] = "▓".yellow().to_string();
            }
            if _render_area[0].contains(&(door.x, door.y)) {
                frame[door.x][door.y] = "▓".yellow().to_string();
            }
        }
        for key in &self.keys {
            if _render_area[1].contains(&(key.x, key.y)) {
                frame[key.x][key.y] = "φ".red().to_string();
            }
            if _render_area[0].contains(&(key.x, key.y)) {
                frame[key.x][key.y] = "φ".red().to_string();
            }
        }
        // for (x, y) in &self.patrol_points {
        //     frame[*x][*y] = "▓".red().to_string();
        // }
    }
}
