use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<String>>;

pub fn new_frame() -> Frame {
    let mut cols: Vec<Vec<String>> = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col: Vec<String> = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ".to_string());
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame, render_area: &Vec<Vec<(usize, usize)>>);
}
