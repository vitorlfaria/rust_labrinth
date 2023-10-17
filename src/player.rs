use crate::{
    NUM_COLS,
    NUM_ROWS,
    frame::{Drawable, Frame}, levels::{level::LevelFactory, wall_tile::WallTile}
};

pub struct Player {
    x: usize,
    y: usize,
    hitbox: Vec<(usize, usize, bool)>,
    pub current_level: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 5,
            hitbox: vec![(1, 0, true), (1, 0, false), (0, 1, true), (0, 1, false)],
            current_level: 1,
        }
    }

    pub fn move_up(&mut self, level: &Vec<WallTile>) {
        let can_move = self.detect_walls(level);
        if self.y > 1 && can_move.1 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, level: &Vec<WallTile>) {
        let can_move = self.detect_walls(level);
        if self.y < NUM_ROWS - 2 && can_move.3 {
            self.y += 1;
        }
    }

    pub fn move_left(&mut self, level: &Vec<WallTile>) {
        let can_move = self.detect_walls(level);
        if self.x > 1 && can_move.0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self, level: &Vec<WallTile>) {
        let can_move = self.detect_walls(level);
        if self.x < NUM_COLS - 2 && can_move.2 {
            self.x += 1;
        }
    }

    pub fn detect_walls(&mut self, level: &Vec<WallTile>) -> (bool, bool, bool, bool) {
        let mut can_move = (true, true, true, true);

        for (x, y, positive) in &self.hitbox {
            for wall in level.iter() {
                if *positive {
                    if *x == 1 && *y == 0 {
                        if self.x + *x == wall.x && self.y + *y == wall.y {
                            can_move.2 = false;
                        }
                    }
                    else if *x == 0 && *y == 1 {
                        if self.x + *x == wall.x && self.y + *y == wall.y {
                            can_move.3 = false;
                        }
                    }
                }
                else {
                    if *x == 1 && *y == 0 {
                        if self.x - *x == wall.x && self.y - *y == wall.y {
                            can_move.0 = false;
                        }
                    }
                    else if *x == 0 && *y == 1 {
                        if self.x - *x == wall.x && self.y - *y == wall.y {
                            can_move.1 = false;
                        }
                    }
                }
            }
        }

        can_move
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "▓".to_string();

        // Draw hitbox
        // for (x, y, positive) in &self.hitbox {
        //     if *positive {
        //         frame[self.x + *x][self.y + *y] = "0".to_string();
        //     }
        //     else {
        //         frame[self.x - *x][self.y - *y] = "0".to_string();
        //     }
        // }
    }
}