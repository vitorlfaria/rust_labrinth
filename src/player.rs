use crate::{
    NUM_COLS,
    NUM_ROWS,
    frame::{Drawable, Frame}
};

pub struct Player {
    x: usize,
    y: usize,
    hitbox: Vec<(usize, usize, bool)>
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 5,
            hitbox: vec![]
        }
    }

    pub fn update(&mut self) {
        self.hitbox = self.create_hitbox();
    }

    pub fn create_hitbox(&self) -> Vec<(usize, usize, bool)> {
        vec![(1, 0, true), (1, 0, false), (0, 1, true), (0, 1, false)]
    }

    pub fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 2 {
            self.x += 1;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.y < NUM_ROWS - 2 {
            self.y += 1;
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "▓".to_string();
        for (x, y, positive) in &self.hitbox {
            if *positive {
                frame[self.x + *x][self.y + *y] = "0".to_string();
            }
            else {
                frame[self.x - *x][self.y - *y] = "0".to_string();
            }
        }
    }
}